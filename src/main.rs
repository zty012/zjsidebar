use zellij_tile::prelude::*;

use std::{collections::BTreeMap, convert::TryInto};

#[derive(Default)]
struct State {
    // the state of the plugin
    tabs: Vec<TabInfo>,
}

register_plugin!(State);

// NOTE: you can start a development environment inside Zellij by running `zellij -l zellij.kdl` in
// this plugin's folder
//
// More info on plugins: https://zellij.dev/documentation/plugins

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // runs once on plugin load, provides the configuration with which this plugin was loaded
        // (if any)
        //
        // this is a good place to `subscribe` (https://docs.rs/zellij-tile/latest/zellij_tile/shim/fn.subscribe.html)
        // to `Event`s (https://docs.rs/zellij-tile/latest/zellij_tile/prelude/enum.Event.html)
        // and `request_permissions` (https://docs.rs/zellij-tile/latest/zellij_tile/shim/fn.request_permission.html)
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::TabUpdate, EventType::Mouse]);
        self.tabs = Vec::new();
    }
    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        // react to `Event`s that have been subscribed to (and the plugin has permissions for)
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself
        if let Event::TabUpdate(ref tabs) = event {
            self.tabs = tabs.to_vec();
            should_render = true;
        }
        if let Event::Mouse(Mouse::LeftClick(line, _col)) = event {
            if line < self.tabs.len().try_into().unwrap() {
                switch_tab_to((line + 1).try_into().unwrap());
            }
        }
        should_render
    }
    fn pipe(&mut self, _pipe_message: PipeMessage) -> bool {
        let should_render = false;
        // react to data piped to this plugin from the CLI, a keybinding or another plugin
        // read more about pipes: https://zellij.dev/documentation/plugin-pipes
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself
        should_render
    }
    fn render(&mut self, rows: usize, cols: usize) {
        // render name of all tabs
        let mut items: Vec<NestedListItem> = Vec::new();
        for (_i, tab) in self.tabs.iter().enumerate() {
            let mut item = NestedListItem::new(tab.name.clone());
            if tab.active {
                item = item.selected();
            }
            items.push(item);
        }
        print_nested_list_with_coordinates(items, 0, 0, Some(cols), Some(rows));
    }
}
