use std::io::Cursor;

use actix_web::{HttpResponse, Responder, post, web};
use exoquant::{Color, ColorMap, Histogram, SimpleColorSpace, generate_palette, optimizer};
use image::{GenericImageView, ImageFormat, io::Reader};
use serde::{Serialize};
use crate::error::GetPaletteResponseError;

#[derive(Serialize)]
struct PaletteColour {
    frequency:usize,
    color:String
}

#[post("/to-palette")]
async fn get_palette(stream:web::Bytes) -> Result<impl Responder,GetPaletteResponseError> {
    let colourspace:SimpleColorSpace = SimpleColorSpace::default();

    // Read image from request
    let image_reader = Reader::new(Cursor::new(stream)).with_guessed_format().map_err(|_| GetPaletteResponseError::BadRequest)?;
    let image = image_reader.decode().map_err(|_| GetPaletteResponseError::BadRequest)?;
    
    // Convert to exoquant image
    let exo_image:Histogram = image.pixels()
                                   .map(|pixel| Color::new(pixel.2[0],pixel.2[1],pixel.2[2],pixel.2[3]))
                                   .collect();

    // Generate palette and colour map
    let palette:Vec<Color> = generate_palette(&exo_image,&colourspace, &optimizer::KMeans, 16);
    let colourmap = ColorMap::new(&palette,&colourspace);

    //Find the frequency of each color in the image
    let mut frequency:Vec<(Color, usize)> = palette.iter()
                                                   .map(|col| (*col,0))
                                                   .collect::<Vec<(Color,usize)>>();

    for cc in exo_image.to_color_counts(&colourspace)
    {
        frequency[colourmap.find_nearest(cc.color)].1 += cc.count;
    }

    // Remove empty frequencies
    frequency.retain(|(_,freq)| *freq != 0);


    // Respond with colour palette
    let out_palette = frequency.iter().map(|(col,freq)| PaletteColour{color:color_to_string(col),frequency:*freq}).collect::<Vec<PaletteColour>>();
    Ok(HttpResponse::Ok().json(out_palette))
}

// Convert exoquant color into hex string
fn color_to_string(color:&Color) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}",color.r,color.g,color.b,color.a)
}