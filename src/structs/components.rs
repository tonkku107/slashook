// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord message components

use serde::{Serialize, Deserialize};
use serde::de;
use serde_json::Value;
use serde_repr::{Serialize_repr, Deserialize_repr};
use super::emojis::Emoji;

#[doc(hidden)]
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ComponentType {
  ACTION_ROW = 1,
  BUTTON = 2,
  SELECT_MENU = 3,
  UNKNOWN
}

/// A component
#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Component {
  ActionRow(ActionRow),
  Button(Box<Button>),
  SelectMenu(SelectMenu),
  Unknown
}

/// A struct for adding components to a message
#[derive(Clone, Debug)]
pub struct Components {
  /// The components
  pub components: Vec<Component>,
}

/// An Action Row component
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionRow {
  #[serde(rename = "type")]
  component_type: ComponentType,
  /// Components inside this row
  pub components: Vec<Component>
}

/// A Button component
///
/// Non-link buttons must have a `custom_id` and cannot have a `url`.\
/// Link buttons must have a `url` and cannot have a `custom_id`.\
/// One of `label` or `emoji` is required.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Button {
  #[serde(rename = "type")]
  component_type: ComponentType,
  /// The style of the Button
  pub style: ButtonStyle,
  /// Text that appears on the button, max 80 characters
  pub label: Option<String>,
  /// An emoji to be shown on the button
  pub emoji: Option<Emoji>,
  /// A developer-defined identifier for the button, max 100 characters
  pub custom_id: Option<String>,
  /// A url for link-style buttons
  pub url: Option<String>,
  /// Whether the button is disabled (default `false`)
  pub disabled: Option<bool>
}

/// Discord Button Styles
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ButtonStyle {
  /// A blurple button
  PRIMARY = 1,
  /// A grey button
  SECONDARY = 2,
  /// A green button
  SUCCESS = 3,
  /// A red button
  DANGER = 4,
  /// A grey button that navigates to a URL
  LINK = 5,
  UNKNOWN
}

/// A Select Menu component
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SelectMenu {
  #[serde(rename = "type")]
  component_type: ComponentType,
  /// A developer-defined identifier for the select menu, max 100 characters
  pub custom_id: Option<String>,
  /// The choices in the select, max 25
  pub options: Vec<SelectOption>,
  /// Custom placeholder text if nothing is selected, max 100 characters
  pub placeholder: Option<String>,
  /// The minimum number of items that must be chosen; default 1, min 0, max 25
  pub min_values: Option<i64>,
  /// The maximum number of items that can be chosen; default 1, max 25
  pub max_values: Option<i64>,
  /// Disable the select, default false
  pub disabled: Option<bool>,
}

/// Choices in a Select Menu
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SelectOption {
  /// The user-facing name of the option, max 100 characters
  pub label: String,
  /// The dev-defined value of the option, max 100 characters
  pub value: String,
  /// An additional description of the option, max 100 characters
  pub description: Option<String>,
  /// An emoji to be shown with the option
  pub emoji: Option<Emoji>,
  /// Will render this option as selected by default
  pub default: Option<bool>
}

impl Components {
  /// Creates a new set of components with an Action Row to start off
  pub fn new() -> Self {
    Self {
      components: vec![Component::ActionRow(ActionRow::new())]
    }
  }

  /// Creates an empty set of components useful for clearing out components when editing a message
  /// ```
  /// # use slashook::commands::{MessageResponse};
  /// # use slashook::structs::components::Components;
  /// let response = MessageResponse::from("Cleared components")
  ///   .set_components(Components::empty());
  /// ```
  pub fn empty() -> Self {
    Self {
      components: Vec::new()
    }
  }

  /// Adds a new row\
  /// A single row has 5 slots of space for components
  /// ```
  /// # use slashook::structs::components::{Components, Button};
  /// let button = Button::new();
  /// let button2 = button.clone();
  /// let components = Components::new()
  ///   .add_button(button)
  ///   .add_row()
  ///   .add_button(button2);
  /// ```
  /// ## Panics
  /// Will panic if you try to add more than the allowed 5 rows
  pub fn add_row(mut self) -> Self {
    if self.components.len() >= 5 {
      panic!("You can only have up to 5 action rows per message.");
    }
    self.components.push(Component::ActionRow(ActionRow::new()));
    self
  }

  /// Adds a button to the last action row\
  /// A button takes up 1 slot of a row
  /// ```
  /// # use slashook::structs::components::{Components, Button};
  /// let button = Button::new();
  /// let components = Components::new()
  ///   .add_button(button);
  /// ```
  /// ## Panics
  /// Will panic if the action row cannot fit any more buttons
  pub fn add_button(mut self, button: Button) -> Self {
    let row = self.components.pop().expect("No action row available");
    if let Component::ActionRow(mut row) = row {
      if row.available_slots() < 1 {
        panic!("The current row doesn't have enough space to contain this component.");
      }
      row.components.push(Component::Button(Box::new(button)));
      self.components.push(Component::ActionRow(row));
    } else {
      panic!("Component is not an Action Row");
    }
    self
  }

  /// Adds a select menu to the last action row\
  /// A select menu takes up 5 slots of a row
  /// ```
  /// # use slashook::structs::components::{Components, SelectMenu};
  /// let select_menu = SelectMenu::new();
  /// let components = Components::new()
  ///   .add_select_menu(select_menu);
  /// ```
  /// ## Panics
  /// Will panic if the action row cannot fit any more select menus
  pub fn add_select_menu(mut self, select_menu: SelectMenu) -> Self {
    let row = self.components.pop().expect("No action row available");
    if let Component::ActionRow(mut row) = row {
      if row.available_slots() < 5 {
        panic!("The current row doesn't have enough space to contain this component.");
      }
      row.components.push(Component::SelectMenu(select_menu));
      self.components.push(Component::ActionRow(row));
    } else {
      panic!("Component is not an Action Row");
    }
    self
  }
}

impl ActionRow {
  /// Creates a new empty action row
  pub fn new() -> Self {
    Self {
      component_type: ComponentType::ACTION_ROW,
      components: Vec::new()
    }
  }

  fn available_slots(&self) -> usize {
    let mut used_slots = 0;
    for component in self.components.iter() {
      match component {
        Component::Button(_) => used_slots += 1,
        Component::SelectMenu(_) => used_slots += 5,
        _ => {}
      }
    }
    5 - used_slots
  }
}

impl Button {
  /// Creates a new button with a primary style by default
  pub fn new() -> Self {
    Self {
      component_type: ComponentType::BUTTON,
      custom_id: None,
      disabled: Some(false),
      style: ButtonStyle::PRIMARY,
      label: None,
      emoji: None,
      url: None
    }
  }

  /// Set the custom_id for a button.\
  /// The command argument is used by the library to choose which command to run when the button is clicked.
  /// The custom_id is formatted as `command/id`
  /// ```
  /// # use slashook::structs::components::Button;
  /// let button = Button::new()
  ///   .set_id("example_button", "cool-button");
  /// assert_eq!(button.custom_id, Some(String::from("example_button/cool-button")));
  /// ```
  pub fn set_id<T: ToString, U: ToString>(mut self, command: T, id: U) -> Self {
    self.custom_id = Some(format!("{}/{}", command.to_string(), id.to_string()));
    self
  }

  /// Set the disabled state of the button
  /// ```
  /// # use slashook::structs::components::Button;
  /// let button = Button::new()
  ///   .set_disabled(true);
  /// assert_eq!(button.disabled, Some(true));
  /// ```
  pub fn set_disabled(mut self, disabled: bool) -> Self {
    self.disabled = Some(disabled);
    self
  }

  /// Set the style of the button
  /// ```
  /// # use slashook::structs::components::{Button, ButtonStyle};
  /// let button = Button::new()
  ///   .set_style(ButtonStyle::DANGER);
  /// assert!(matches!(button.style, ButtonStyle::DANGER));
  /// ```
  pub fn set_style(mut self, style: ButtonStyle) -> Self {
    self.style = style;
    self
  }

  /// Set the label of the button
  /// ```
  /// # use slashook::structs::components::Button;
  /// let button = Button::new()
  ///   .set_label("Cool button");
  /// assert_eq!(button.label, Some(String::from("Cool button")));
  /// ```
  pub fn set_label<T: ToString>(mut self, label: T) -> Self {
    self.label = Some(label.to_string());
    self
  }

  /// Set the emoji of the button
  /// ```
  /// # use slashook::structs::components::Button;
  /// # use slashook::structs::Emoji;
  /// let button = Button::new()
  ///   .set_emoji(Emoji::new_standard_emoji("👌🏻"));
  /// assert_eq!(button.emoji.unwrap().name, Some(String::from("👌🏻")));
  /// ```
  pub fn set_emoji(mut self, emoji: Emoji) -> Self {
    self.emoji = Some(emoji);
    self
  }

  /// Set the url for a link-style button
  /// ```
  /// # use slashook::structs::components::{Button, ButtonStyle};
  /// let button = Button::new()
  ///   .set_style(ButtonStyle::LINK)
  ///   .set_url("https://example.com");
  /// assert_eq!(button.url, Some(String::from("https://example.com")));
  /// ```
  pub fn set_url<T: ToString>(mut self, url: T) -> Self {
    self.url = Some(url.to_string());
    self
  }
}

impl SelectMenu {
  /// Creates a new select menu
  pub fn new() -> Self {
    Self {
      component_type: ComponentType::SELECT_MENU,
      custom_id: None,
      disabled: Some(false),
      options: Vec::new(),
      placeholder: None,
      min_values: None,
      max_values: None
    }
  }

  /// Set the custom_id for a select menu.\
  /// The command argument is used by the library to choose which command to run when the select menu is updated.
  /// The custom_id is formatted as `command/id`
  /// ```
  /// # use slashook::structs::components::SelectMenu;
  /// let select_menu = SelectMenu::new()
  ///   .set_id("example_select", "choice");
  /// assert_eq!(select_menu.custom_id, Some(String::from("example_select/choice")));
  /// ```
  pub fn set_id<T: ToString, U: ToString>(mut self, command: T, id: U) -> Self {
    self.custom_id = Some(format!("{}/{}", command.to_string(), id.to_string()));
    self
  }

  /// Set the disabled state of the select menu
  /// ```
  /// # use slashook::structs::components::SelectMenu;
  /// let select_menu = SelectMenu::new()
  ///   .set_disabled(true);
  /// assert_eq!(select_menu.disabled, Some(true));
  /// ```
  pub fn set_disabled(mut self, disabled: bool) -> Self {
    self.disabled = Some(disabled);
    self
  }

  /// Add a choice to the select menu
  /// ```
  /// # use slashook::structs::components::{SelectMenu, SelectOption};
  /// let select_menu = SelectMenu::new()
  ///   .add_option(SelectOption::new("First choice", "1"))
  ///   .add_option(SelectOption::new("Second choice", "2"));
  /// ```
  pub fn add_option(mut self, option: SelectOption) -> Self {
    self.options.push(option);
    self
  }

  /// Set the placeholder of the select menu
  /// ```
  /// # use slashook::structs::components::SelectMenu;
  /// let select_menu = SelectMenu::new()
  ///   .set_placeholder("Choose an option");
  /// assert_eq!(select_menu.placeholder, Some(String::from("Choose an option")));
  /// ```
  pub fn set_placeholder<T: ToString>(mut self, placeholder: T) -> Self {
    self.placeholder = Some(placeholder.to_string());
    self
  }

  /// Set the minimum required choices for a select menu
  /// ```
  /// # use slashook::structs::components::SelectMenu;
  /// let select_menu = SelectMenu::new()
  ///   .set_min_values(2);
  /// assert_eq!(select_menu.min_values, Some(2));
  /// ```
  pub fn set_min_values(mut self, min_values: i64) -> Self {
    self.min_values = Some(min_values);
    self
  }

  /// Set the maximum amount of choices for a select menu
  /// ```
  /// # use slashook::structs::components::SelectMenu;
  /// let select_menu = SelectMenu::new()
  ///   .set_max_values(5);
  /// assert_eq!(select_menu.max_values, Some(5));
  /// ```
  pub fn set_max_values(mut self, max_values: i64) -> Self {
    self.max_values = Some(max_values);
    self
  }
}

impl SelectOption {
  /// Creates a new choice for a select menu with a label and value
  /// ```
  /// # use slashook::structs::components::SelectOption;
  /// let option = SelectOption::new("Option label", "Option value");
  /// ```
  pub fn new<T: ToString, U: ToString>(label: T, value: U) -> Self {
    Self {
      label: label.to_string(),
      value: value.to_string(),
      description: None,
      emoji: None,
      default: Some(false)
    }
  }

  /// Set the description for a choice
  /// ```
  /// # use slashook::structs::components::SelectOption;
  /// let option = SelectOption::new("Option label", "Option value")
  ///   .set_description("This is an option");
  /// ```
  pub fn set_description<T: ToString>(mut self, description: T) -> Self {
    self.description = Some(description.to_string());
    self
  }

  /// Set the emoji for a choice
  /// ```
  /// # use slashook::structs::components::SelectOption;
  /// # use slashook::structs::Emoji;
  /// let option = SelectOption::new("Option label", "Option value")
  ///   .set_emoji(Emoji::new_standard_emoji("👌🏻"));
  /// ```
  pub fn set_emoji(mut self, emoji: Emoji) -> Self {
    self.emoji = Some(emoji);
    self
  }

  /// Set the default state of a choice
  /// ```
  /// # use slashook::structs::components::SelectOption;
  /// let option = SelectOption::new("Option label", "Option value")
  ///   .set_default(true);
  /// ```
  pub fn set_default(&mut self, default: bool) -> &Self {
    self.default = Some(default);
    self
  }
}

impl Default for Components {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for ActionRow {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for Button {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for SelectMenu {
  fn default() -> Self {
    Self::new()
  }
}

impl<'de> serde::Deserialize<'de> for Component {
  fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let value = Value::deserialize(d)?;

    Ok(match value.get("type").and_then(Value::as_u64).ok_or_else(|| de::Error::custom("Expected a field \"type\" of type u64"))? {
      1 => Component::ActionRow(ActionRow::deserialize(value).map_err(de::Error::custom)?),
      2 => Component::Button(Box::new(Button::deserialize(value).map_err(de::Error::custom)?)),
      3 => Component::SelectMenu(SelectMenu::deserialize(value).map_err(de::Error::custom)?),
      _ => Component::Unknown,
    })
  }
}
