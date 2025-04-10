use iced::advanced::widget::Tree;
use iced::advanced::{Layout, Widget, layout, renderer};
use iced::widget::image;
use iced::{Element, Length, Rectangle, Size, Theme, mouse, widget};

#[derive(Debug)]
pub struct BackgroundImage {
    image_handle: widget::image::Handle,
}

impl BackgroundImage {
    pub fn new(image_handle: widget::image::Handle) -> Self {
        Self { image_handle }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for BackgroundImage
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = image::Handle>,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        iced::widget::image::layout(
            renderer,
            limits,
            &self.image_handle,
            Length::Fill,
            Length::Fill,
            iced::ContentFit::Contain,
            iced::Rotation::Solid(0.into()),
        )
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        iced::widget::image::draw(
            renderer,
            layout,
            viewport,
            &self.image_handle,
            iced::ContentFit::Contain,
            iced::widget::image::FilterMethod::Nearest,
            iced::Rotation::Solid(0.into()),
            1.,
            1.,
        );
    }
}

impl<Message, Renderer> From<BackgroundImage> for Element<'_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = image::Handle>,
{
    fn from(widget: BackgroundImage) -> Self {
        Self::new(widget)
    }
}
