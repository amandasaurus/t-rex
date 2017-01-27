//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

use cache::cache::Cache;
use std::fs::{self,File};
use std::io::{self,Read,Write};
use std::path::Path;


pub struct Filecache {
    pub basepath: String,
}

impl Filecache {
    fn path_for_tile(&self, tileset_name: &str, zoom: u8, x: u16, y: u16) -> String {
        let x1 = format!("{:03}", x/1_000);
        let x2 = format!("{:03}", x % 1_000);
        let y1 = format!("{:03}", y/1_000);
        let y2 = format!("{:03}", y % 1_000);

        format!("{}/{}/{}/{}/{}/{}/{}.pbf", self.basepath, tileset_name, zoom, x1, x2, y1, y2)
    }
}

impl Cache for Filecache {
    fn read<F>(&self, tileset_name: &str, zoom: u8, x: u16, y: u16, mut read: F) -> bool
        where F : FnMut(&mut Read)
    {
        let fullpath = self.path_for_tile(tileset_name, zoom, x, y);
        debug!("Filecache.read {}", fullpath);
        match File::open(&fullpath) {
            Ok(mut f) => { read(&mut f); true },
            Err(_e) => false
        }
    }
    fn write(&self, tileset_name: &str, zoom: u8, x: u16, y: u16, obj: &[u8]) -> Result<(), io::Error>
    {
        let fullpath = self.path_for_tile(tileset_name, zoom, x, y);
        debug!("Filecache.write {}", fullpath);
        let p = Path::new(&fullpath);
        try!(fs::create_dir_all(p.parent().unwrap()));
        let mut f = try!(File::create(&fullpath));
        f.write_all(obj)
    }

    fn exists(&self, tileset_name: &str, zoom: u8, x: u16, y: u16) -> bool
    {
        let fullpath = self.path_for_tile(tileset_name, zoom, x, y);
        Path::new(&fullpath).exists()
    }
}

#[test]
fn test_dircache() {
    use std::env;

    let mut dir = env::temp_dir();
    dir.push("t_rex_test");
    let basepath = format!("{}", &dir.display());
    let _ = fs::remove_dir_all(&basepath);

    let cache = Filecache { basepath: basepath };
    let tileset_name = "tileset";
    let zoom = 0;
    let x = 1;
    let y = 2;
    let path = "tileset/0/1/2.pbf";
    let fullpath = format!("{}/{}", cache.basepath, path);
    let obj = "0123456789";

    // Cache miss
    assert_eq!(cache.read(tileset_name, zoom, x, y, |_| {}), false);

    // Write into cache
    let _ = cache.write(tileset_name, zoom, x, y, obj.as_bytes());
    assert!(Path::new(&fullpath).exists());

    // Cache hit
    assert_eq!(cache.read(tileset_name, zoom, x, y, |_| {}), true);

    // Read from cache
    let mut s = String::new();
    cache.read(tileset_name, zoom, x, y, |f| {
        let _ = f.read_to_string(&mut s);
    });
    assert_eq!(&s, "0123456789");
}
