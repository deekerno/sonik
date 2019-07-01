use std::convert::AsRef;
use std::iter::{self, Iterator};

use unicode_width::UnicodeWidthStr;

use tui::buffer::Buffer;
use tui::layout::{Corner, Rect};
use tui::style::Style;
use tui::widgets::{Block, List, Text, Widget};

use crate::database::record::Record;

pub struct RecordList<'b> {
    block: Option<Block<'b>>,
    // If this doesn't work automatically, generate a vec of artist names
    items: Vec<&'b str>,
    selected: Option<usize>,
    style: Style,
    highlight_style: Style,
    highlight_symbol: Option<&'b str>,
}

impl<'b> Default for RecordList<'b> {
    fn default() -> RecordList<'b> {
        RecordList {
            block: None,
            items: Vec::new(),
            selected: None,
            style: Default::default(),
            highlight_style: Default::default(),
            highlight_symbol: None,
        }
    }
}

impl<'b> RecordList<'b> {
    pub fn block(mut self, block: Block<'b>) -> RecordList<'b> {
        self.block = Some(block);
        self
    }

    pub fn items<I: Record>(mut self, items: &'b [I]) -> RecordList<'b> {
        self.items = items.into_iter().map(|a| a.name()).collect::<Vec<&str>>();
        self
    }

    pub fn style(mut self, style: Style) -> RecordList<'b> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'b str) -> RecordList<'b> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> RecordList<'b> {
        self.highlight_style = highlight_style;
        self
    }

    pub fn select(mut self, index: Option<usize>) -> RecordList<'b> {
        self.selected = index;
        self
    }
}

impl<'b> Widget for RecordList<'b> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let list_area = match self.block {
            Some(ref mut b) => b.inner(area),
            None => area,
        };

        let list_height = list_area.height as usize;

        // Use highlight_style only if something is selected
        let (selected, highlight_style) = match self.selected {
            Some(i) => (Some(i), self.highlight_style),
            None => (None, self.style),
        };
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = iter::repeat(" ")
            .take(highlight_symbol.width())
            .collect::<String>();
        // Make sure the list show the selected item
        let offset = if let Some(selected) = selected {
            if selected >= list_height {
                selected - list_height + 1
            } else {
                0
            }
        } else {
            0
        };

        // Render items
        let items = self
            .items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                if let Some(s) = selected {
                    if i == s {
                        Text::styled(format!("{} {}", highlight_symbol, item), highlight_style)
                    } else {
                        Text::styled(format!("{} {}", blank_symbol, item), self.style)
                    }
                } else {
                    Text::styled(item, self.style)
                }
            })
            .skip(offset as usize);
        List::new(items)
            .block(self.block.unwrap_or_default())
            .style(self.style)
            .draw(area, buf);
    }
}
