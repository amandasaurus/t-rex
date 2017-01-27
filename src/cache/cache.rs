//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

use std::io::Read;
use std::io;


pub trait Cache {
    fn read<F>(&self, tileset_name: &str, zoom: u8, x: u16, y: u16, read: F) -> bool
        where F : FnMut(&mut Read);
    fn write(&self, tileset_name: &str, zoom: u8, x: u16, y: u16, obj: &[u8]) -> Result<(), io::Error>;
    fn exists(&self, tileset_name: &str, zoom: u8, x: u16, y: u16) -> bool;
}


pub struct Nocache;

impl Cache for Nocache {
     #[allow(unused_variables)]
    fn read<F>(&self, tileset_name: &str, zoom: u8, x: u16, y: u16, read: F) -> bool
        where F : FnMut(&mut Read)
    {
        false
    }
     #[allow(unused_variables)]
    fn write(&self, tileset_name: &str, zoom: u8, x: u16, y: u16, obj: &[u8]) -> Result<(), io::Error>
    {
        Ok(())
    }

     #[allow(unused_variables)]
    fn exists(&self, tileset_name: &str, zoom: u8, x: u16, y: u16) -> bool
    {
        false
    }
}
