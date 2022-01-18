use {
    std::path::Path,
    ggez07::{
        Context,
        ContextBuilder,
        GameError,
        GameResult,
        conf::{
            ModuleConf,
            WindowMode,
            WindowSetup,
        },
        event::EventHandler,
        filesystem,
        graphics::{
            self,
            Align as HorizontalAlign,
            Color,
            DrawParam,
            Drawable as _,
            Font,
            PxScale,
            Rect,
            Text,
            TextFragment,
            Transform,
        },
        timer,
    },
};

#[cfg(target_os = "linux")] const DEJAVU_PATH: &str = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
#[cfg(target_os = "windows")] const DEJAVU_PATH: &str = "\\Windows\\Fonts\\DejaVuSans.ttf";

enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

struct TextBox {
    text: Text,
    size: f32,
    halign: HorizontalAlign,
    valign: VerticalAlign,
}

impl TextBox {
    fn new(text: impl Into<TextFragment>) -> Self {
        Self {
            text: Text::new(text),
            ..Self::default()
        }
    }

    fn halign(mut self, halign: HorizontalAlign) -> Self {
        self.halign = halign;
        self
    }

    fn valign(mut self, valign: VerticalAlign) -> Self {
        self.valign = valign;
        self
    }

    fn draw(mut self, handler: &crate::Handler, ctx: &mut Context) -> GameResult {
        let Rect { w, h, .. } = graphics::screen_coordinates(ctx);
        self.text.set_font(handler.dejavu_sans, PxScale::from(self.size));
        self.text.set_bounds([w - self.size, h - self.size], self.halign);
        let param = DrawParam::default().dest([
            match self.halign {
                HorizontalAlign::Left => self.size / 2.0,
                HorizontalAlign::Center => w / 2.0,
                HorizontalAlign::Right => w - self.size / 2.0,
            },
            match self.valign {
                VerticalAlign::Top => self.size / 2.0,
                VerticalAlign::Middle => h / 2.0,
                VerticalAlign::Bottom => h - self.size / 2.0,
            },
        ]).offset([
            match self.halign {
                HorizontalAlign::Left => 0.0,
                HorizontalAlign::Center => 0.5,
                HorizontalAlign::Right => 1.0,
            },
            match self.valign {
                VerticalAlign::Top => 0.0,
                VerticalAlign::Middle => 0.5,
                VerticalAlign::Bottom => 1.0,
            },
        ]);
        let mut new_param = param;
        if let Transform::Values { offset, .. } = param.trans {
            let dim = self.text.dimensions(ctx);
            let new_offset = mint::Vector2 {
                x: offset.x * dim.w + dim.x,
                y: offset.y * dim.h + dim.y,
            };
            new_param = param.offset(new_offset);
        }
        self.text.draw(ctx, new_param)
    }
}

impl Default for TextBox {
    fn default() -> Self {
        Self {
            text: Text::default(),
            size: 100.0,
            halign: HorizontalAlign::Center,
            valign: VerticalAlign::Middle,
        }
    }
}

struct Handler {
    dejavu_sans: Font,
}

impl Handler {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            dejavu_sans: Font::new(ctx, DEJAVU_PATH)?,
        })
    }
}

impl EventHandler<GameError> for Handler {
    fn update(&mut self, _: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (w, h) = graphics::drawable_size(ctx);
        graphics::set_screen_coordinates(ctx, Rect { x: 0.0, y: 0.0, w, h })?;
        graphics::clear(ctx, Color::BLACK);
        TextBox::new("upper left").halign(HorizontalAlign::Left).valign(VerticalAlign::Top).draw(self, ctx)?;
        TextBox::new("upper center").halign(HorizontalAlign::Center).valign(VerticalAlign::Top).draw(self, ctx)?;
        TextBox::new("upper right").halign(HorizontalAlign::Right).valign(VerticalAlign::Top).draw(self, ctx)?;
        TextBox::new("middle left").halign(HorizontalAlign::Left).valign(VerticalAlign::Middle).draw(self, ctx)?;
        TextBox::new("middle center").halign(HorizontalAlign::Center).valign(VerticalAlign::Middle).draw(self, ctx)?;
        TextBox::new("middle right").halign(HorizontalAlign::Right).valign(VerticalAlign::Middle).draw(self, ctx)?;
        TextBox::new("bottom left").halign(HorizontalAlign::Left).valign(VerticalAlign::Bottom).draw(self, ctx)?;
        TextBox::new("bottom center").halign(HorizontalAlign::Center).valign(VerticalAlign::Bottom).draw(self, ctx)?;
        TextBox::new("bottom right").halign(HorizontalAlign::Right).valign(VerticalAlign::Bottom).draw(self, ctx)?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, evt_loop) = ContextBuilder::new("ex07", "Fenhl")
        .window_setup(WindowSetup {
            title: format!("ggez 0.7 example"),
            ..WindowSetup::default()
        })
        .window_mode(WindowMode {
            resizable: true,
            ..WindowMode::default()
        })
        .modules(ModuleConf {
            gamepad: false,
            audio: false,
        })
        .build()?;
    #[cfg(windows)] filesystem::mount(&mut ctx, Path::new("C:\\"), true); // for font support
    #[cfg(not(windows))] filesystem::mount(&mut ctx, Path::new("/"), true); // for font support
    let handler = Handler::new(&mut ctx)?;
    ggez07::event::run(ctx, evt_loop, handler)
}
