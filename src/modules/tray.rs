use crate::{
    components::icons::{icon, Icons},
    services::{
        tray::{TrayData, TrayService},
        ServiceEvent,
    },
    style::header_pills,
};
use iced::{
    widget::{container, Image, Row},
    Alignment, Element, Length,
};

#[derive(Debug, Clone)]
pub enum TrayMessage {
    Event(ServiceEvent<TrayService>),
}

impl TrayData {
    pub fn view(&self) -> Element<TrayMessage> {
        container(
            Row::with_children(
                self.iter()
                    .map(|item| {
                        if let Some(pixmap) = &item.icon_pixmap {
                            Image::new(pixmap.clone()).height(Length::Fixed(14.)).into()
                        } else {
                            icon(Icons::Point).into()
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .padding([2, 0])
            .align_y(Alignment::Center)
            .spacing(8),
        )
        .padding([2, 8])
        .style(header_pills)
        .into()
    }
}
