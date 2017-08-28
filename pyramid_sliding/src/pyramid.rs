//! This module contains struct and functions that aid in the loading and manipulation of
//! pyramids.
use std;

/// A struct that represents a pyramid held in memory.
pub struct Pyramid {
    /// The actual pyramid data packed into a 1-dimensional array.
    /// Each level contains the one more item that it's level number.
    pub data: Vec<usize>,
    /// The number of levels to this data.
    pub height: usize,
}

/// Print the pyramid out in a "pretty" way.
#[allow(unused)] // Could be a useful function to keep around.
pub fn print_pyramid(pyramid: &Pyramid) {
    let mut index = 0;
    // For each height level starting from the bottom.
    for current_level in 0..pyramid.height {
        let width = level_width(current_level);
        for _ in 0..width {
            print!("{} ", pyramid.data[index]);
            index += 1;
        }
        println!();
    }
}

/// Load a pyramid from a filepath.
pub fn load_pyramid(path: &str) -> Result<Pyramid, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(path).map_err(to_err)?;
    let mut reader = BufReader::new(file);

    let mut s = String::new();
    reader.read_line(&mut s).map_err(to_err)?;
    let height = s.trim().parse::<usize>().map_err(to_err)?;
    let size = calc_pyramid_size(height);
    let mut data = Vec::with_capacity(size);

    for _ in 0..height {
        s.clear();
        reader.read_line(&mut s).map_err(to_err)?;
        for value in s.split_whitespace() {
            let value = value.parse::<usize>().map_err(to_err)?;
            data.push(value);
        }
    }

    Ok(Pyramid {
        data,
        height,
    })
}

/// The location of a block in the pyramid.
pub struct Location {
    /// The level this location is found at.
    pub level: usize,
    /// The block number on the level this location is at.
    pub block: usize,
}

/// Retreives the left choice from the current location.
pub fn left_choice(current: &Location) -> Location {
    Location {
        level: current.level + 1,
        block: current.block,
    }
}

/// Retreives the right choice from the current location.
pub fn right_choice(current: &Location) -> Location {
    Location {
        level: current.level + 1,
        block: current.block + 1,
    }
}

/// Gets the cost of a specific location in the pyramid.
pub fn cost_of(pyramid: &Pyramid, location: &Location) -> usize {
    let index = index(location);
    pyramid.data[index]
}

/// Get the 1d-array index of the specified location.
fn index(location: &Location) -> usize {
    let mut index = 0;
    for level in 0..location.level {
        index += level_width(level);
    }
    index += location.block;
    index
}

/// Retreive the level width of a particular level.
fn level_width(level: usize) -> usize {
    level + 1
}

/// Calulates the pyramid size based on the number of levels of the pyramid.
fn calc_pyramid_size(mut n: usize) -> usize {
    // pyramid_size(n) = pyramid_size(n-1) + n
    let mut size = 0;
    while n > 0 {
        size += n;
        n -= 1;
    }
    size
}

/// The error type for loading a pyramid.
pub type Error = String;

/// Converts anything displayable to our error type.
fn to_err<T: std::fmt::Display>(err: T) -> Error {
    format!("{}", err)
}
