use std::io::Cursor;
use actix_web::{HttpResponse, Responder, post, web};
use exoquant::{Color, ColorMap, Histogram, SimpleColorSpace, generate_palette, optimizer};
use image::{GenericImageView, io::Reader};
use serde::{Deserialize, Serialize};
use crate::error::GetPaletteResponseError;


#[derive(Serialize)]
struct PaletteResponse {
    frequency:usize,
    color:String
}


#[derive(Deserialize)]
struct Params{
    num_colors:Option<usize>,
    ignore_white:Option<bool>,
    ignore_black:Option<bool>,
    ignore_transparent:Option<bool>,
    use_transparency:Option<bool>
}


#[post("/to-palette")]
async fn get_palette(stream:web::Bytes, params:web::Query<Params>) -> Result<impl Responder,GetPaletteResponseError> {
    let colourspace:SimpleColorSpace = SimpleColorSpace::default();

    // Extract parameter values and set defaults
    let num_colors = params.num_colors.unwrap_or(4).min(256);
    let ignore_white = params.ignore_white.unwrap_or(true);
    let ignore_black = params.ignore_black.unwrap_or(true);
    let ignore_transparent = params.ignore_transparent.unwrap_or(true);
    let use_transparency = params.use_transparency.unwrap_or(true);

    // Read image from request
    let image_reader = Reader::new(Cursor::new(stream)).with_guessed_format().map_err(|_|GetPaletteResponseError::BadRequest(1, "Couldn't guess format".to_string()))?;
    let image = image_reader.decode().map_err(|_| GetPaletteResponseError::BadRequest(2, "Couldn't decode image".to_string()))?;
    
    // Convert to exoquant image
    // Set alpha to 255 if use_transparancy is false
    let exo_image:Histogram = image.pixels()
                                   .map(|pixel| Color::new(pixel.2[0],pixel.2[1],pixel.2[2],if use_transparency {pixel.2[3]} else {255}))
                                   .collect();

    // Generate palette and colour map
    let palette:Vec<Color> = generate_palette(&exo_image,&colourspace, &optimizer::KMeans, num_colors);
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

    // Remove mostly white colors
    if ignore_white{
        frequency.retain(|(col,_)| !(col.r > 232 && col.g > 232 && col.b > 232));
    }

    // Remove mostly black colors
    if ignore_black{
        frequency.retain(|(col,_)| !(col.r < 15 && col.g < 15 && col.b < 15));
    }

    // Remove mostly transparent colors
    if ignore_transparent{
        frequency.retain(|(col,_)| col.a > 10);
    }

    // Respond with colour palette
    let out_palette = frequency.iter()
                               .map(|(col,freq)| PaletteResponse{color:color_to_string(col),frequency:*freq})
                               .collect::<Vec<PaletteResponse>>();
    Ok(HttpResponse::Ok().json(out_palette))
}


// Convert exoquant color into hex string
fn color_to_string(color:&Color) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}",color.r,color.g,color.b,color.a)
}