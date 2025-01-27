use pmdk::PMem;

const BUFFER_SIZE: usize = 4096;
const DEST_FILEPATH: &str = "/pmem0/pmempool0\0";
const TEXT: &str = " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec dictum, massa sit amet tempus blandit, mi purus suscipit arcu, a egestas erat orci et ipsum. Phasellus vel urna non urna cursus imperdiet. Aliquam turpis ex, maximus id tortor eget, tincidunt feugiat metus. Ut ultrices auctor massa, quis convallis lectus vulputate et. Maecenas at mi orci. Donec id leo vitae risus tempus imperdiet ut a elit. Mauris quis dolor urna. Mauris dictum enim vel turpis aliquam tincidunt. Pellentesque et eros ac quam lobortis hendrerit non ut nulla. Quisque maximus magna tristique risus lacinia, et facilisis erat molestie.

Morbi eget sapien accumsan, rhoncus metus in, interdum libero. Nam gravida mi et viverra porttitor. Sed malesuada odio semper sapien bibendum ornare. Curabitur scelerisque lacinia ex, a rhoncus magna viverra eu. Maecenas sed libero vel ex dictum congue at sed nulla. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam erat volutpat. Proin condimentum augue eu nulla consequat efficitur. Vivamus sodales pretium erat, id iaculis risus pellentesque sit amet. Integer tempus porta diam ac facilisis. Duis ex eros, mattis nec ultrices vel, varius vel lectus. Proin varius sapien est, nec euismod ex varius nec. Quisque in sem sit amet metus scelerisque ornare at a nisi. Maecenas ac scelerisque metus. In ut velit placerat, fringilla eros non, semper risus. Cras sed ante maximus, vestibulum nunc nec, rutrum leo. \0";
const TEXT2: &str = "hello world!";

fn basic_read_write_test() {
    unsafe {
        let mut is_pmem: i32 = 0;
        let mut mapped_len: u64 = 0;

        let mut pmem = match PMem::create(
            &DEST_FILEPATH,
            64 * 1024 * 1024 * 1024,
            &mut mapped_len,
            &mut is_pmem,
        ) {
            Ok(value) => value,
            Err(_) => match PMem::open(&DEST_FILEPATH, &mut mapped_len, &mut is_pmem) {
                Ok(value) => value,
                Err(_) => panic!("\n Failed to create or open pmem file handle."),
            },
        };

        // Writing the long text (TEXT1)
        let mut text_array = [0u8; BUFFER_SIZE];
        TEXT.bytes()
            .zip(text_array.iter_mut())
            .for_each(|(b, ptr)| *ptr = b);
        pmem.write(0, &text_array, TEXT.chars().count())
            .expect("\n Failed to write TEXT1 to pmem file handle!");

        // Writing the short text (TEXT2)
        TEXT2
            .bytes()
            .zip(text_array.iter_mut())
            .for_each(|(b, ptr)| *ptr = b);
        pmem.write(TEXT.chars().count(), &text_array, TEXT2.chars().count())
            .expect("\n Failed to write TEXT2 to pmem file handle!");

        // Reading the long text (TEXT1)
        let mut buffer = vec![0; TEXT.chars().count()];
        pmem.read(0, &mut buffer, TEXT.chars().count() as u64)
            .expect("Failed to read TEXT1 from pmem file handle!");

        // Reading the short text (TEXT2)
        let mut buffer2 = vec![0; TEXT2.chars().count()];
        pmem.read(
            TEXT.chars().count(),
            &mut buffer2,
            TEXT2.chars().count() as u64,
        )
        .expect("Failed to read TEXT2 from pmem file handle!");

        // Writing the long text (TEXT1) starting offset 1000
        TEXT.bytes()
            .zip(text_array.iter_mut())
            .for_each(|(b, ptr)| *ptr = b);
        pmem.write(1000, &text_array, TEXT.chars().count())
            .expect("Failed to write TEXT1 with offset to pmem file handle!");

        // Reading the recently text
        let mut buffer3 = vec![0; TEXT.chars().count()];
        pmem.read(1000, &mut buffer3, TEXT.chars().count() as u64)
            .expect("Failed to read TEXT1 with offset from pmem file handle!");

        pmem.close(&mapped_len);

        // Comparing the read text with the actual one
        let read_string = match std::str::from_utf8(&buffer) {
            Ok(string) => string,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(TEXT, read_string);

        let read_string2 = match std::str::from_utf8(&buffer2) {
            Ok(string) => string,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(TEXT2, read_string2);

        let read_string3 = match std::str::from_utf8(&buffer3) {
            Ok(string) => string,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        assert_eq!(TEXT, read_string3);

        println!("Successfully written and read text to/from PMDK!");
    }
}

fn main() {
    basic_read_write_test();
}
