use std::{collections::HashMap, io::Read, path::PathBuf};

use enum_iterator::all;
use ggez::{context::Has, graphics::{GraphicsContext, Image}};

use crate::pieces::{Color as PieceColor, Piece, PieceType};



pub struct AssetManager {
    asset_dir: PathBuf,
    piece_images: HashMap<(PieceType, PieceColor), Vec<u8>>,
}

impl AssetManager {
    pub fn new(asset_dir: PathBuf) -> Self {
        let mut this = Self { asset_dir, piece_images: HashMap::new() };
        let mut piece_images = HashMap::new();
        for piece_type in all::<PieceType>() {
            if piece_type == PieceType::Empty {
                continue;
            }
            for color in all::<PieceColor>() {
                let file_name = this.file_name(piece_type, color);
                // Open files as bytes
                let file = std::fs::File::open(&file_name).unwrap();
                let bytes = file.bytes().map(|v| v.unwrap()).collect::<Vec<_>>();
                piece_images.insert((piece_type, color), bytes);
            }
        }
        this.piece_images = piece_images;
        this
    }   
    pub fn image(&self, piece: &Piece, gfx: &impl Has<GraphicsContext>) -> Image {
        let bytes = self.piece_images.get(&(piece.piece_type, piece.color)).unwrap();
        Image::from_bytes(gfx, bytes).unwrap()
    }
    fn file_name(&self, piece_type: PieceType, color: PieceColor) -> PathBuf {
        let color_str = match color {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        };
        let filename = format!("{}_{:#}.png", color_str, piece_type);
        self.asset_dir.join(filename)
    }
}