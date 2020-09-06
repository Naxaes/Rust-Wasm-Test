use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

pub struct Texture {

}

impl Texture {
    pub fn from_pixels(gl: &GL, width: usize, height: usize, pixels: Option<&[u8]>) -> Result<WebGlTexture, String> {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            /* target */         GL::TEXTURE_2D,
            /* level */          0,
            /* internalformat */ GL::RGBA as i32,
            /* width */          width as i32,
            /* height */         height as i32,
            /* border */         0,
            /* format */         GL::RGBA,
            /* type_ */          GL::UNSIGNED_BYTE,
            /* pixels */         pixels,
        ).unwrap();

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S,     GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T,     GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);

        //  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_BASE_LEVEL, 0);
        //  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAX_LEVEL,  1000);  // 1000 is the default.
        //  glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_LOD_BIAS, options.lod_bias);
        //  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, options.min);
        //  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, options.mag);
        //  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, options.wrap_s);
        //  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, options.wrap_t);
        // //  vec4 border_color(1.0f, 1.0f, 0.0f, 1.0f);
        // //  glTexParameterfv(GL_TEXTURE_2D, GL_TEXTURE_BORDER_COLOR, &border_color[0]);
        //  glTexImage2D(GL_TEXTURE_2D, 0, options.internal, width, height, 0, format, GL_UNSIGNED_BYTE, data);
        //  glGenerateMipmap(GL_TEXTURE_2D);  // NOTE(ted): This has to be called after glTexImage2D!

        //     return { .id = texture, .type = type, .height = height, .width = width, .name = name, .channels = channels, .dimension = 2 };
        Ok(texture)
    }
}


