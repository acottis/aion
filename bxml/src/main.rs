use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    writer::Writer,
};
use std::{collections::HashMap, ffi::CStr, fs::File, io::Read};

/// Deserialise any value that implements .from_le_bytes() onto the given buffer
/// ```
/// let mut len = 0;
/// let mut buf = [0u8; 100];
///
/// let id = consume_le_bytes!(len, buf, u16);
/// ```
#[macro_export]
macro_rules! consume_le_bytes {
    ($ptr:expr, $buf:expr, $ty:ty) => {{
        let size = core::mem::size_of::<$ty>();
        let ret =
            <$ty>::from_le_bytes($buf[$ptr..$ptr + size].try_into().unwrap());
        $ptr += size;
        ret
    }};
}

#[macro_export]
macro_rules! consume_varint {
    ($ptr:expr, $buf:expr) => {{
        let mut varint = 0;
        let mut i = 0;

        loop {
            let byte = consume_le_bytes!($ptr, $buf, u8);
            let remaining_bits = (byte & 0x7F) as usize;
            let shifted_bits = remaining_bits << (i * 7);
            varint ^= shifted_bits;

            i += 1;
            if (byte & 0x80) != 0x80 {
                break;
            }
        }

        varint
    }};
}

struct HeaderTable {
    table: String,
}

impl<'str> HeaderTable {
    fn new(buf: &[u8]) -> Self {
        let mut table = String::new();
        for bytes in buf.chunks(2) {
            table.push(bytes[0] as char);
        }

        Self { table }
    }

    fn get_cstr(&'str self, i: usize) -> &'str CStr {
        CStr::from_bytes_until_nul(&self.table.as_bytes()[i..]).unwrap()
    }
    fn get_str(&'str self, i: usize) -> &'str str {
        CStr::from_bytes_until_nul(&self.table.as_bytes()[i..])
            .unwrap()
            .to_str()
            .unwrap()
    }
}

fn main() {
    let mut file =
        File::open("C:/tmp/out/data/items/client_special_skin_mesh_name.xml")
            .unwrap();
    let mut file =
        File::open("C:/tmp/out/data/items/client_items.xml").unwrap();

    let mut buf = Vec::new();

    _ = file.read_to_end(&mut buf);

    let mut len = 0;
    let magic = consume_le_bytes!(len, buf, u8);
    assert_eq!(magic, 0x80, "Wrong file magic");

    let table_len = consume_varint!(len, buf);
    let table = HeaderTable::new(&buf[len..len + table_len]);
    len += table_len;

    let mut writer = Writer::new_with_indent(
        std::io::Cursor::new(Vec::<u8>::with_capacity(1_000_000)),
        b' ',
        2,
    );
    to_xml(&mut buf, &mut len, &table, &mut writer);
    let xml = String::from_utf8(writer.into_inner().into_inner()).unwrap();
    println!("{xml}",);
}

fn to_xml(
    buf: &[u8],
    len: &mut usize,
    table: &HeaderTable,
    writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
) {
    let index = consume_varint!(*len, buf);
    let name = table.get_str(index);
    let ty = consume_varint!(*len, buf);
    writer
        .write_event(Event::Start(BytesStart::new(name)))
        .unwrap();

    match ty {
        1 => {
            let index = consume_varint!(*len, buf);
            let value = table.get_str(index);
            writer
                .write_event(Event::Text(BytesText::new(value)))
                .unwrap();
        }
        4 => {
            let count = consume_varint!(*len, buf);
            for _ in 0..count {
                to_xml(buf, len, table, writer);
            }
        }
        _ => unreachable!(),
    }
    writer.write_event(Event::End(BytesEnd::new(name))).unwrap();
}

#[derive(Debug)]
struct Node<'node> {
    name: &'node CStr,
    value: Option<&'node CStr>,
    children: Vec<Node<'node>>,
    attributes: HashMap<&'node CStr, &'node CStr>,
}

impl<'node> Node<'node> {
    pub fn new(name: &'node CStr) -> Self {
        Self {
            name,
            children: Vec::new(),
            attributes: HashMap::new(),
            value: None,
        }
    }

    fn parse(
        buf: &[u8],
        len: &mut usize,
        table: &'node HeaderTable,
    ) -> Node<'node> {
        let index = consume_varint!(*len, buf);
        let name = table.get_cstr(index);
        let ty = consume_varint!(*len, buf);
        let mut node = Node::new(name);

        match ty {
            1 => {
                let index = consume_varint!(*len, buf);
                node.value = Some(table.get_cstr(index));
            }
            4 => {
                let count = consume_varint!(*len, buf);
                for _ in 0..count {
                    node.children.push(Self::parse(buf, len, table));
                }
            }
            _ => unreachable!(),
        }

        node
    }
}
