pub enum Target {
    Screen { width: u32, height: u32 },
    Pixels { texture: three_d::Texture2D },
}

impl Target {
    pub fn clear(&mut self, context: &three_d::Context, clear_state: three_d::ClearState) -> &Self {
        match self {
            Target::Screen { width, height } => {
                three_d::RenderTarget::screen(context, *width, *height).clear(clear_state);
            }
            Target::Pixels { texture, .. } => {
                texture.as_color_target(None).clear(clear_state);
            }
        }
        self
    }
    pub fn write<E: std::error::Error>(
        &mut self,
        context: &three_d::Context,
        render: impl FnOnce() -> Result<(), E>,
    ) -> Result<(), E> {
        match self {
            Target::Screen { width, height } => {
                three_d::RenderTarget::screen(context, *width, *height).write(render)?;
            }
            Target::Pixels { texture, .. } => {
                texture.as_color_target(None).write(render)?;
            }
        }
        Ok(())
    }
    pub fn pixels(&mut self) -> Vec<u8> {
        match self {
            Target::Pixels { texture, .. } => texture
                .as_color_target(None)
                .read::<[u8; 4]>()
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
            Target::Screen { .. } => unreachable!(),
        }
    }
}
