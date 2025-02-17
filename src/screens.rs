// Aqui está todo a interface do KC Overlay, o design e as mensagens produzidas por cada widget.

use iced::{
    theme,
    widget::{column, container, row, text, Column},
    Alignment, Color, Font, Length, Renderer,
};

use crate::{
    stats::StatsType,
    themed_widgets::{
        button, button_with_color, pick_list, red_button, secondary_button, slider, text_input, toggler,
    },
    util, Message, MineClient,
};

#[derive(Clone, Copy, Default, Debug)]
pub enum Screen {
    #[default]
    Main,
    Settings,
    Welcome,
    Info,
    ViewPlayer,
}

pub fn get_screen(
    screen: Screen,
    app: &super::KCOverlay,
) -> Column<'static, super::Message, theme::Theme, Renderer> {
    const COLUMN_HEIGHT: u16 = 390;

    match screen {
        Screen::Main => {
            let screen_title_text = if app.players.is_empty() {
                String::from(
                    "Digite o comando /jogando no chat do Mush para ver os stats dos jogadores",
                )
            } else if app.loading {
                String::from("Carregando jogadores...")
            } else if app.waiting > 0 {
                format!("Espere {} segundos para usar novamente", app.waiting)
            } else {
                format!(
                    "Top {} jogadores da sala ({})",
                    app.players.len(),
                    app.stats_type
                )
            };

            let screen_title_widget = text(screen_title_text);

            let bar = row![screen_title_widget];

            let mut username_column = Column::new().width(300);
            let mut winstreak_column = Column::new().align_x(Alignment::Center);
            let mut winrate_column = Column::new().align_x(Alignment::Center);
            let mut fkdr_column = Column::new().align_x(Alignment::Center);
            let mut kdr_column = Column::new().align_x(Alignment::Center);

            let players = app.players.clone();

            if !players.is_empty() {
                username_column = username_column.push(text("Nome"));
                winstreak_column = winstreak_column.push(text("WS"));
                winrate_column = winrate_column.push(text("WLR"));
                fkdr_column = fkdr_column.push(text("FKDR"));
                kdr_column = kdr_column.push(text("KDR"));
            }
            for player in players {
                let (
                    level,
                    level_symbol,
                    level_color,
                    winstreak,
                    winrate,
                    final_kill_death_ratio,
                    kill_death_ratio,
                ) = match player.stats {
                    crate::stats::Stats::Bedwars(bedwars) => (
                        bedwars.level,
                        bedwars.level_symbol,
                        bedwars.level_color,
                        bedwars.winstreak,
                        bedwars.winrate,
                        bedwars.final_kill_death_ratio,
                        bedwars.kill_death_ratio,
                    ),
                };
                let clan = if let Some(value) = &player.clan {
                    format!("[{}]", value)
                } else {
                    String::new()
                };

                let level_widget = if player.is_nicked {
                    row![text("[NICKED]").color(Color::from_rgb8(255, 255, 0))]
                } else if player.is_possible_cheater {
                    row![text("[possível CHEATER]")
                        .color(Color::from_rgb8(255, 0, 0))
                        .size(12)]
                } else {
                    row![
                        text(format!("[{}", level)).color(level_color.to_color()),
                        text(level_symbol)
                            .font(Font::with_name("Noto Sans Symbols 2"))
                            .color(level_color.to_color()),
                        text("]").color(level_color.to_color())
                    ]
                };

                let username_widget = text(player.username).color(player.username_color.to_color());
                let clan_widget = text(clan).color(player.clan_color.to_color());
                let (winstreak_widget, winrate_widget, fkdr, kdr) = if player.is_nicked {
                    (text("?"), text("?"), text("?"), text("?"))
                } else {
                    (
                        text(format!("{}", winstreak)),
                        text(format!("{:.2}", winrate)),
                        text(format!("{:.2}", final_kill_death_ratio)),
                        text(format!("{:.2}", kill_death_ratio)),
                    )
                };

                let username_row = row![level_widget, username_widget, clan_widget].spacing(5);

                username_column = username_column.push(username_row);
                winstreak_column = winstreak_column.push(winstreak_widget);
                winrate_column = winrate_column.push(winrate_widget);
                fkdr_column = fkdr_column.push(fkdr);
                kdr_column = kdr_column.push(kdr)
            }
            let column_row = row![
                username_column,
                winstreak_column,
                winrate_column,
                fkdr_column,
                kdr_column
            ]
            .spacing(15);
            let container = container(column_row);

            let settings_btn = if app.rgb_buttons {
                button_with_color("Configurações", app.get_rgb_color(0.0))
            } else {
                button("Configurações")
            };
            let settings = settings_btn.on_press(Message::ChangeScreen(Screen::Settings));

            let view_player = if app.rgb_buttons {
                button_with_color("Ver jogador", app.get_rgb_color(0.2))
            } else {
                button("Ver jogador")
            }.on_press(Message::ChangeScreen(Screen::ViewPlayer));

            let info = if app.rgb_buttons {
                button_with_color("Sobre", app.get_rgb_color(0.4))
            } else {
                button("Sobre")
            }.on_press(Message::ChangeScreen(Screen::Info));

            let close = if app.rgb_buttons {
                button_with_color("Sair", app.get_rgb_color(0.8))
            } else {
                red_button("Sair")
            }.on_press(Message::Close);

            let minimize = if app.rgb_buttons {
                button_with_color("Minimizar", app.get_rgb_color(0.6))
            } else {
                button("Minimizar")
            }.on_press(Message::Minimize);

            let mut left_bottom_row = row![settings, view_player, info]
                .spacing(15)
                .width(Length::Fill);
            let right_bottom_row = row![minimize, close].spacing(15);
            if app.update.available {
                let update_button = secondary_button("Atualizar").on_press(Message::Update);
                left_bottom_row = left_bottom_row.push(update_button);
            }

            let bottom_row = row![left_bottom_row, right_bottom_row].spacing(20);

            let main_column = column![bar, container].spacing(10).height(COLUMN_HEIGHT);

            column![main_column, bottom_row].padding(10).spacing(10)
        }
        Screen::Settings => {
            use super::MineClient;
            let clients = vec![
                MineClient::Default,
                MineClient::Badlion,
                MineClient::Lunar,
                MineClient::LegacyLauncher,
                MineClient::Silent,
                MineClient::Custom(" ".to_string()),
            ];

            let client_select = pick_list(clients, Some(app.client.clone()), Message::ClientSelect);
            let client_row = row![text("Client:"), client_select].spacing(10);

            let stats_select = pick_list(
                StatsType::get_stats_list(),
                Some(app.stats_type.clone()),
                Message::StatsSelect,
            );
            let stats_row = row![text("Stats:"), stats_select].spacing(10);

            let go_back = button("Voltar").on_press(Message::ChangeScreen(Screen::Main));

            let mut main_column = column![client_row, stats_row]
                .spacing(20)
                .height(COLUMN_HEIGHT);

            if let MineClient::Custom(path) = &app.client {
                let custom_client_text = text("Último log do client (ex: logs/latest.log):");
                let custom_client_path = text_input("Insira o último arquivo de log", path)
                    .on_input(Message::CustomClientPathModified);
                let custom_client_search = button("Procurar").on_press(Message::SearchExplorer);

                let custom_client_row = row![custom_client_path, custom_client_search].spacing(10);

                main_column = main_column
                    .push(column![custom_client_text, custom_client_row])
                    .spacing(20)
            }

            let never_minimize_toggler = toggler(app.never_minimize)
                .on_toggle(Message::ChangeNeverMinimize)
                .size(20);

            let never_minimize_text = text("Nunca minimizar automaticamente");
            let never_minimize_row = row![never_minimize_toggler, never_minimize_text].spacing(10);

            main_column = main_column.push(never_minimize_row);

            let seconds_to_minimize_text = text(format!(
                "Mostrar o KC Overlay por {} segundos após carregar os jogadores",
                app.seconds_to_minimize
            ));
            let seconds_to_minimize_slider = slider(
                5.0..=120.,
                app.seconds_to_minimize as f64,
                Message::ChangeSecondsToMinimize,
            )
            .width(240);
            let seconds_to_minimize_column =
                column![seconds_to_minimize_text, seconds_to_minimize_slider].spacing(5);

            if !app.never_minimize {
                main_column = main_column.push(seconds_to_minimize_column)
            }

            let auto_manage_players_toggler = toggler(app.auto_manage_players)
                .on_toggle(Message::ChangeRemoveEliminatedPlayers)
                .size(20);
            let auto_manage_players_text =
                text("Adicionar e remover jogadores automaticamente (ainda é necessário digitar o comando antes da partida iniciar)");
            let auto_manage_players_row =
                row![auto_manage_players_toggler, auto_manage_players_text].spacing(10);

            let window_scale_slider = slider(
                50.0..=125.,
                app.window_scale * 100.,
                Message::WindowScaleChanged,
            );
            let window_scale_row = row![
                text(format!("Tamanho da janela ({}x):", app.window_scale)),
                window_scale_slider
            ]
            .spacing(10);

            let rgb_buttons_toggler = toggler(app.rgb_buttons)
                .on_toggle(Message::ChangeRGBButtons)
                .size(20);
            let rgb_buttons_text = text("Botões RGB");
            let rgb_buttons_row = row![rgb_buttons_toggler, rgb_buttons_text].spacing(10);
            
            main_column = main_column.push(rgb_buttons_row);

            main_column = main_column.push(auto_manage_players_row);
            main_column = main_column.push(window_scale_row);

            column![main_column, go_back].padding(10).spacing(10)
        }
        Screen::Welcome => {
            let welcome_text = text("Muito obrigado por usar a overlay! Selecione o client que você usa para proseguir.");

            let default_client =
                button("Geral / outro").on_press(Message::ClientSelect(MineClient::Default));
            let default_client_text = text("Vanilla, CMClient, Forge, etc");
            let dafault_client_row = row![default_client, default_client_text].spacing(10);

            let badlion = button("Badlion").on_press(Message::ClientSelect(MineClient::Badlion));

            let lunar = button("Lunar").on_press(Message::ClientSelect(MineClient::Lunar));

            let legacy_launcher = button("Legacy Launcher")
                .on_press(Message::ClientSelect(MineClient::LegacyLauncher));

            let silent_client =
                button("Silent Client").on_press(Message::ClientSelect(MineClient::Silent));

            column![
                welcome_text,
                dafault_client_row,
                badlion,
                lunar,
                legacy_launcher,
                silent_client
            ]
            .spacing(10)
            .padding(10)
        }
        Screen::Info => {
            let thanks_text = text("Muito obrigado por usar o KC Overlay! Considere virar membro do Discord para saber das novidades");
            let discord_button = button("Entrar no Discord").on_press(Message::OpenLink(
                String::from("https://discord.gg/SqT7YHSGzJ"),
            ));

            let creditos = text(format!(
                "KC Overlay {} - Criado por Jafkc2 - Desenvolvedor: oRvdy",
                env!("CARGO_PKG_VERSION")
            ));
            let github = button("Acessar Github").on_press(Message::OpenLink(String::from(
                "https://github.com/jafkc2/KC-Overlay",
            )));

            let go_back = button("Voltar").on_press(Message::ChangeScreen(Screen::Main));

            let discord_column = column![thanks_text, discord_button].spacing(10).height(125);
            let credits_column = column![creditos, github].spacing(10);

            let main_column = column![discord_column, credits_column].height(COLUMN_HEIGHT);

            column![main_column, go_back].spacing(10).padding(10)
        }
        Screen::ViewPlayer => {
            let input = text_input("Nome do jogador", &app.player_to_view_username)
                .on_input(Message::ViewPlayerInputChanged);
            let stats_pick_list = pick_list(
                StatsType::get_stats_list(),
                Some(app.searched_player_stats_type.clone()),
                Message::ViewPlayerStatsChanged,
            );
            let search_player = button("Ver stats").on_press(Message::ViewPlayer);
            let input_row = row![input, stats_pick_list, search_player].spacing(10);
            let mut main_column = column![input_row].height(COLUMN_HEIGHT).spacing(20);

            let go_back = button("Voltar").on_press(Message::ChangeScreen(Screen::Main));

            if let Some(player) = &app.searched_player {
                let connected = match player.is_connected {
                    true => text("Sim").color(Color::from_rgb8(166, 218, 149)),
                    false => text("Não").color(Color::from_rgb8(237, 135, 150)),
                };

                let connected_row = row![text("Online:"), connected].spacing(10);

                let first_login_date = format!(
                    "Primeiro login: {}.",
                    util::unix_time_to_date(player.account_creation)
                );
                let first_login_widget = text(first_login_date);
                let last_login_date = format!(
                    "Último login: {}.",
                    util::unix_time_to_date(player.last_login)
                );
                let last_login_widget = text(last_login_date);

                let clan = if let Some(value) = &player.clan {
                    format!("[{}]", value)
                } else {
                    String::new()
                };

                let username_widget =
                    text(player.username.clone()).color(player.username_color.to_color());
                let clan_widget = text(clan).color(player.clan_color.to_color());

                let player_column = match &player.stats {
                    crate::stats::Stats::Bedwars(bedwars) => {
                        let hours_played = text(format!("Horas jogadas: {}", bedwars.hours_played));

                        let level_widget = row![
                            text(format!("[{}", bedwars.level))
                                .color(bedwars.level_color.to_color()),
                            text(bedwars.level_symbol.clone())
                                .font(Font::with_name("Noto Sans Symbols 2"))
                                .color(bedwars.level_color.to_color()),
                            text("]").color(bedwars.level_color.to_color())
                        ];
                        let (
                            winstreak_widget,
                            winrate_widget,
                            fkdr,
                            kdr,
                            wins,
                            losses,
                            final_kills,
                            final_deaths,
                            kills,
                            deaths,
                            assists,
                        ) = (
                            text(format!("Winstreak: {}", bedwars.winstreak)),
                            text(format!("WLR: {:.2}", bedwars.winrate)),
                            text(format!("FKDR: {:.2}", bedwars.final_kill_death_ratio)),
                            text(format!("KDR: {:.2}", bedwars.kill_death_ratio)),
                            text(format!("Vitórias: {}", bedwars.wins)),
                            text(format!("Derrotas: {}", bedwars.losses)),
                            text(format!("Final kills: {}", bedwars.final_kills)),
                            text(format!("Final deaths: {}", bedwars.final_deaths)),
                            text(format!("Kills: {}", bedwars.kills)),
                            text(format!("Mortes: {}", bedwars.deaths)),
                            text(format!("Assistências: {}", bedwars.assists)),
                        );

                        let username_row =
                            row![level_widget, username_widget, clan_widget].spacing(5);

                        let left_column = column![
                            connected_row,
                            first_login_widget,
                            last_login_widget,
                            hours_played
                        ]
                        .spacing(10);
                        let middle_column =
                            column![winstreak_widget, winrate_widget, fkdr, kdr].spacing(10);
                        let right_column = column![
                            wins,
                            losses,
                            final_kills,
                            final_deaths,
                            kills,
                            deaths,
                            assists
                        ]
                        .spacing(10);

                        column![
                            username_row,
                            row![left_column, middle_column, right_column].spacing(60)
                        ]
                        .spacing(10)
                    }
                };

                main_column = main_column.push(player_column);
            }

            column![main_column, go_back].spacing(10).padding(10)
        }
    }
}
