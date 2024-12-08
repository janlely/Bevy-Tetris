use bevy::prelude::*;

pub struct DropdownPlugin;

#[derive(Component)]
pub struct Dropdown {
    pub id: String,
    pub options: Vec<String>,
    pub selected: usize,
    pub is_open: bool
}

#[derive(Event)]
pub struct DropdownSelectedEvent {
    pub enity: Entity,
    pub id: String,
    pub selected: usize
}

#[derive(Component)]
struct CurrentOption;

impl Dropdown {
    pub fn new(id: String, options: Vec<String>, selected: usize) -> Self {
        Self {
            id,
            options,
            selected,
            is_open: false
        }
    }
}

pub struct DropdownPlugin;

impl Plugin for DropdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropdownSelectedEvent>()
        .add_systems(Update, )
    }
}

fn dropdown_render(
    mut commands: Commands,
    mut dropdowns: Query<(Entity, Option<&CurrentOption>, &mut Dropdown, &Interaction)>,
) {
    for (entity, cur_option, &mut dropdown, &interaction) in dropdowns.iter_mut() {
        if cur_option.is_none() {
            commands.entity(entity)
            .insert(CurrentOption(dropdown.options[dropdown.selected].clone()))
            .insert((
                Text::new(dropdown.options[dropdown.selected].clone()),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    ..default()
                },
                ..default()
            ));
        }
        match interaction {
            Interaction::Pressed => {
                dropdown.is_open = !dropdown.is_open;
                if dropdown.is_open {
                    spawn_dropdown(&mut commands, entity, dropdown);
                } else {
                    clear_dropdown(&mut commands, entity);
                }
            }
        }
    }
}

fn spawn_dropdown(
    commands: &mut Commands,
    entity: Entity,
    dropdown: Dropdown
) {
    commands.entity(entity).with_children(|parent| {
        for (idx, option) in dropdown.options.iter().enumerate() {
            parent.spawn((
                Text::new(option.clone()),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    ..default()
                },
                Interaction::default(),
                ..default()
            ));
        }
    })
}

fn handle_dropdown_selection(
    mut commands: Commands,
    dropdowns: Query<(Entity,&Dropdown)>,
    mut ev_select: EventReader<DropdownSelectedEvent>,
) {
}

// pub fn setup_dropdown(&mut commands: Commands, config: Dropdown) {
//     commands.spawn(Node {
//         flex_direction: FlexDirection::Row,
//         ..Default::default()
//     })
//     .with_children(|parent| {
//         //Label
//         parent.spawn((
//             Text::new(format!("{}:", config.label)),
//             TextFont {
//                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                 font_size: 42.0,
//                 ..default()
//             },
//             ..Default::default()));
//         //selected text with border
//         parent.spawn(Node {
//             flex_direction: FlexDirection::Column,
//             ..Default::default()
//         })
//         .with_children(|dropdown| {
//             dropdown.spawn((
//                 Text::new(format!("{config.options[config.selected]}")),
//                 TextFont {
//                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 42.0,
//                     ..default()
//                 },
//                 TextColor {
//                     color: Color::BLACK,
//                 },
//                 TextAlignment {
//                     horizontal: HorizontalAlign::Center,
//                     ..default()
//                 },
//                 ..Default::default()
//             ));
//             dropdown.spawn(Node {
//                 style: Style {
//                     flex_direction: FlexDirection::Column,
//                     ..default()
//                 },
//                 ..default()
//             }).with_children(|scroller| {
//                 for i in 0..config.options.len() {
//                     scroller.spawn((
//                         Text(format!("{config.options[i]}")),
//                         TextFont {
//                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                             ..default()
//                         },
//                         Label,
//                         AccessibilityNode(Accessible::new(Role::ListItem)),
//                     ))
//                 }
//             });
//         })
//     });
// }

