
use graphics::types::{ Color, FontSize };
use graphics::{ color, Image, Graphics, Transformed, DrawState };
use graphics::character::CharacterCache;
use graphics::math::Matrix2d;

/// Renders text
#[derive(Copy, Clone)]
pub struct Text {
    /// The color
    pub color: Color,
    /// The font size
    pub font_size: FontSize,
}

impl Text {
    /// Creates a new text with black color
    pub fn new(font_size: FontSize) -> Text {
        Text {
            color: color::BLACK,
            font_size: font_size,
        }
    }

    /// Creates a new colored text
    pub fn colored(
        color: Color,
        font_size: FontSize
    ) -> Text {
        Text {
            color: color,
            font_size: font_size,
        }
    }

    /// Draws text with a character cache
    pub fn draw<C, G>(
        &self,
        text: &str,
        cache: &mut C,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where
            C: CharacterCache,
            G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        let image = Image::new_colored(self.color);
        let mut x = 0.0;
        let mut y = 0.0;

        for ch in text.chars() {
            let character = cache.character(self.font_size, ch);
            image.draw(&character.texture,
                draw_state,
                transform.trans(
                    x,
                    y
                ),
                g
            );
            x += character.width();
            y += character.height();
        }
    }
}
