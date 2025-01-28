// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::*;

#[test]
fn test_vec_writer() {
    let mut writer = Vec::new();
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6, 7]).unwrap(), 4);
    let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(writer, b);
}

#[test]
fn test_mem_writer() {
    let mut writer = Cursor::new(Vec::new());
    writer.set_position(10);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6, 7]).unwrap(), 4);
    let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(&writer.get_ref()[..10], &[0; 10]);
    assert_eq!(&writer.get_ref()[10..], b);
}

#[test]
fn test_mem_writer_preallocated() {
    let mut writer = Cursor::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10]);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6, 7]).unwrap(), 4);
    let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(&writer.get_ref()[..], b);
}

#[test]
fn test_mem_mut_writer() {
    let mut vec = Vec::new();
    let mut writer = Cursor::new(&mut vec);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6, 7]).unwrap(), 4);
    let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(&writer.get_ref()[..], b);
}

fn test_slice_writer<T>(writer: &mut Cursor<T>)
where
    T: AsRef<[u8]>,
    Cursor<T>: Write,
{
    assert_eq!(writer.position(), 0);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.position(), 1);
    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6, 7]).unwrap(), 4);
    assert_eq!(writer.position(), 8);
    assert_eq!(writer.write(&[]).unwrap(), 0);
    assert_eq!(writer.position(), 8);

    assert_eq!(writer.write(&[8, 9]).unwrap(), 1);
    assert_eq!(writer.write(&[10]).unwrap(), 0);
    let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(writer.get_ref().as_ref(), b);
}

#[test]
fn test_box_slice_writer() {
    let mut writer = Cursor::new(vec![0u8; 9].into_boxed_slice());
    test_slice_writer(&mut writer);
}

#[test]
fn test_array_writer() {
    let mut writer = Cursor::new([0u8; 9]);
    test_slice_writer(&mut writer);
}

#[test]
fn test_buf_writer() {
    let mut buf = [0 as u8; 9];
    let mut writer = Cursor::new(&mut buf[..]);
    test_slice_writer(&mut writer);
}

#[test]
fn test_buf_writer_seek() {
    let mut buf = [0 as u8; 8];
    {
        let mut writer = Cursor::new(&mut buf[..]);
        assert_eq!(writer.position(), 0);
        assert_eq!(writer.write(&[1]).unwrap(), 1);
        assert_eq!(writer.position(), 1);

        assert_eq!(writer.seek(SeekFrom::Start(2)).unwrap(), 2);
        assert_eq!(writer.position(), 2);
        assert_eq!(writer.write(&[2]).unwrap(), 1);
        assert_eq!(writer.position(), 3);

        assert_eq!(writer.seek(SeekFrom::Current(-2)).unwrap(), 1);
        assert_eq!(writer.position(), 1);
        assert_eq!(writer.write(&[3]).unwrap(), 1);
        assert_eq!(writer.position(), 2);

        assert_eq!(writer.seek(SeekFrom::End(-1)).unwrap(), 7);
        assert_eq!(writer.position(), 7);
        assert_eq!(writer.write(&[4]).unwrap(), 1);
        assert_eq!(writer.position(), 8);
    }
    let b: &[_] = &[1, 3, 2, 0, 0, 0, 0, 4];
    assert_eq!(buf, b);
}

#[test]
fn test_buf_writer_error() {
    let mut buf = [0 as u8; 2];
    let mut writer = Cursor::new(&mut buf[..]);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.write(&[0, 0]).unwrap(), 1);
    assert_eq!(writer.write(&[0, 0]).unwrap(), 0);
}
