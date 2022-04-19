use {
    std::path::Path,
    ggezwgpu::{
        Context,
        ContextBuilder,
        GameError,
        GameResult,
        conf::{
            WindowMode,
            WindowSetup,
        },
        event::EventHandler,
        graphics::{
            Canvas,
            CanvasLoadOp,
            Color,
            DrawParam,
            Drawable as _,
            FontData,
            PxScale,
            Rect,
            Text,
            TextAlign,
            TextFragment,
            TextLayout,
        },
        timer,
    },
};

#[cfg(target_os = "linux")] const DEJAVU_PATH: &str = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
#[cfg(target_os = "windows")] const DEJAVU_PATH: &str = "\\Windows\\Fonts\\DejaVuSans.ttf";

struct TextBox {
    text: Text,
    size: f32,
    halign: TextAlign,
    valign: TextAlign,
}

impl TextBox {
    fn new(text: impl Into<TextFragment>) -> Self {
        Self {
            text: Text::new(text),
            ..Self::default()
        }
    }

    fn halign(mut self, halign: TextAlign) -> Self {
        self.halign = halign;
        self
    }

    fn valign(mut self, valign: TextAlign) -> Self {
        self.valign = valign;
        self
    }

    fn draw(mut self, canvas: &mut Canvas) -> GameResult<()> {
        let Rect { w, h, .. } = canvas.screen_coordinates().expect("set in Handler::draw");
        self.text.set_font("DejaVu Sans");
        self.text.set_scale(PxScale::from(self.size));
        self.text.set_bounds([w - self.size, h - self.size], TextLayout::Wrap { h_align: self.halign, v_align: self.valign });
        self.text.draw(canvas, DrawParam::default().dest([self.size / 2.0, self.size / 2.0]));
        Ok(())
    }
}

impl Default for TextBox {
    fn default() -> Self {
        Self {
            text: Text::default(),
            size: 100.0,
            halign: TextAlign::Middle,
            valign: TextAlign::Middle,
        }
    }
}

struct Handler;

impl EventHandler<GameError> for Handler {
    fn update(&mut self, _: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(&ctx.gfx, CanvasLoadOp::Clear(Color::BLACK));
        let (w, h) = ctx.gfx.drawable_size();
        canvas.set_screen_coordinates(Rect { x: 0.0, y: 0.0, w, h });
        TextBox::new("upper left").halign(TextAlign::Begin).valign(TextAlign::Begin).draw(&mut canvas)?;
        TextBox::new("upper center").halign(TextAlign::Middle).valign(TextAlign::Begin).draw(&mut canvas)?;
        TextBox::new("upper right").halign(TextAlign::End).valign(TextAlign::Begin).draw(&mut canvas)?;
        TextBox::new("middle left").halign(TextAlign::Begin).valign(TextAlign::Middle).draw(&mut canvas)?;
        TextBox::new("middle center").halign(TextAlign::Middle).valign(TextAlign::Middle).draw(&mut canvas)?;
        TextBox::new("middle right").halign(TextAlign::End).valign(TextAlign::Middle).draw(&mut canvas)?;
        TextBox::new("bottom left").halign(TextAlign::Begin).valign(TextAlign::End).draw(&mut canvas)?;
        TextBox::new("bottom center").halign(TextAlign::Middle).valign(TextAlign::End).draw(&mut canvas)?;
        TextBox::new("bottom right").halign(TextAlign::End).valign(TextAlign::End).draw(&mut canvas)?;
        canvas.finish(&mut ctx.gfx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, evt_loop) = ContextBuilder::new("exwgpu", "Fenhl")
        .window_setup(WindowSetup {
            title: format!("ggez wgpu branch example"),
            ..WindowSetup::default()
        })
        .window_mode(WindowMode {
            resizable: true,
            ..WindowMode::default()
        })
        .build()?;
    #[cfg(windows)] ctx.fs.mount(Path::new("C:\\"), true); // for font support
    #[cfg(not(windows))] ctx.fs.mount(Path::new("/"), true); // for font support
    ctx.gfx.add_font("DejaVu Sans", FontData::from_path(&ctx.fs, DEJAVU_PATH)?);
    ggezwgpu::event::run(ctx, evt_loop, Handler)
}
