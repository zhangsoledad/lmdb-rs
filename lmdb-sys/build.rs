extern crate gcc;

fn main() {
    gcc::Build::new()
            .file("lmdb/libraries/liblmdb/mdb.c")
            .file("lmdb/libraries/liblmdb/midl.c")
            .opt_level(2)
            .compile("iblmdb.a");
}
