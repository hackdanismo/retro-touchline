//! Retro Touchline — a CM-era inspired football manager UI mock-up.
//! UI prototype with formations, mentality, XI, bench and named custom tactics.
//! No persistence, email service, or football simulation.
//! Real Chelsea 2001/02 roster; player attributes and contracts are illustrative.

use eframe::egui::{self, Color32, RichText, Stroke, Pos2, Rect, Vec2, Sense, Align2};
use std::time::Instant;

const BG: Color32 = Color32::from_rgb(15, 35, 29);
const PANEL: Color32 = Color32::from_rgb(23, 54, 44);
const PANEL_ALT: Color32 = Color32::from_rgb(30, 67, 54);
const HEADER: Color32 = Color32::from_rgb(11, 44, 37);
const GOLD: Color32 = Color32::from_rgb(226, 192, 103);
const WHITE: Color32 = Color32::from_rgb(226, 234, 221);
const MUTED: Color32 = Color32::from_rgb(151, 176, 158);
const DIVIDER: Color32 = Color32::from_rgb(68, 99, 79);
// Chelsea-inspired sidebar: primary Chelsea blue and a lighter active row.
const CHELSEA_BLUE: Color32 = Color32::from_rgb(3, 70, 148); // #034694
const CHELSEA_BLUE_ACTIVE: Color32 = Color32::from_rgb(26, 96, 172);
const SIDEBAR_WHITE: Color32 = Color32::WHITE;

struct Player {
    number: u8,
    name: &'static str,
    position: &'static str,
    nationality: &'static str,
    age: u8,
    apps: u8,
    goals: u8,
    rating: &'static str,
    condition: &'static str,
}

// Real 2001/02 Chelsea squad; ages calculated for 18 August 2001.
// Matches, ratings and condition have no simulated values yet.
const PLAYERS: &[Player] = &[
    Player { number: 1, name: "Ed de Goey", position: "GK", nationality: "NED", age: 34, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 3, name: "Celestine Babayaro", position: "D L", nationality: "NGA", age: 22, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 6, name: "Marcel Desailly", position: "D C", nationality: "FRA", age: 32, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 7, name: "Winston Bogarde", position: "D L", nationality: "NED", age: 30, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 8, name: "Frank Lampard", position: "M C", nationality: "ENG", age: 23, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 9, name: "Jimmy Floyd Hasselbaink", position: "S C", nationality: "NED", age: 29, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 10, name: "Slavisa Jokanovic", position: "DM C", nationality: "YUG", age: 33, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 11, name: "Boudewijn Zenden", position: "AM L", nationality: "NED", age: 25, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 12, name: "Mario Stanic", position: "M R", nationality: "CRO", age: 29, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 13, name: "William Gallas", position: "D C", nationality: "FRA", age: 24, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 14, name: "Graeme Le Saux", position: "D L", nationality: "ENG", age: 32, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 15, name: "Mario Melchiot", position: "D R", nationality: "NED", age: 24, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 17, name: "Emmanuel Petit", position: "DM C", nationality: "FRA", age: 30, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 18, name: "Albert Ferrer", position: "D R", nationality: "ESP", age: 31, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 19, name: "Valerio di Cesare", position: "D C", nationality: "ITA", age: 18, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 20, name: "Jody Morris", position: "M C", nationality: "ENG", age: 22, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 22, name: "Eidur Gudjohnsen", position: "S C", nationality: "ISL", age: 22, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 23, name: "Carlo Cudicini", position: "GK", nationality: "ITA", age: 27, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 24, name: "Samuele Dalla Bona", position: "M C", nationality: "ITA", age: 20, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 25, name: "Gianfranco Zola", position: "AM/F C", nationality: "ITA", age: 35, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 26, name: "John Terry", position: "D C", nationality: "ENG", age: 20, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 27, name: "Rati Aleksidze", position: "S C", nationality: "GEO", age: 23, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 28, name: "Leon Knight", position: "S C", nationality: "ENG", age: 18, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 29, name: "Robert Huth", position: "D C", nationality: "GER", age: 17, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 30, name: "Jesper Gronkjaer", position: "AM R", nationality: "DEN", age: 24, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 31, name: "Mark Bosnich", position: "GK", nationality: "AUS", age: 29, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 32, name: "Mikael Forssell", position: "S C", nationality: "FIN", age: 20, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 33, name: "Joel Kitamirike", position: "D C", nationality: "UGA", age: 17, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 34, name: "Pat Baldwin", position: "D C", nationality: "ENG", age: 18, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 35, name: "Rhys Evans", position: "GK", nationality: "ENG", age: 19, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 36, name: "Joe Keenan", position: "M L", nationality: "ENG", age: 18, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 37, name: "Danny Slatter", position: "D L", nationality: "WAL", age: 20, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 38, name: "Scott Cousins", position: "D C", nationality: "ENG", age: 18, apps: 0, goals: 0, rating: "—", condition: "—" },
    Player { number: 39, name: "Carlton Cole", position: "S C", nationality: "ENG", age: 17, apps: 0, goals: 0, rating: "—", condition: "—" },
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Overview, Squad, Tactics, Training, Fixtures, LeagueTable,
    Inbox, Transfers, Scouting, Finances, Staff, Manager, Match, Results,
}

impl Screen {
    const CLUB: [Self; 6] = [Self::Overview, Self::Squad, Self::Tactics, Self::Training, Self::Fixtures, Self::LeagueTable];
    const MANAGEMENT: [Self; 6] = [Self::Inbox, Self::Transfers, Self::Scouting, Self::Finances, Self::Staff, Self::Manager];
    fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview", Self::Squad => "Squad", Self::Tactics => "Tactics",
            Self::Training => "Training", Self::Fixtures => "Fixtures", Self::LeagueTable => "Competitions",
            Self::Inbox => "Inbox", Self::Transfers => "Transfers", Self::Scouting => "Scouting",
            Self::Finances => "Finances", Self::Staff => "Staff", Self::Manager => "Manager", Self::Match => "Match Centre", Self::Results => "Matchday Results",
        }
    }
}

struct InboxMessage {
    sender: &'static str,
    subject: &'static str,
    date: &'static str,
    preview: &'static str,
    body: &'static str,
}

// Fictional messages for the UI prototype, not real historical club correspondence.
const MESSAGES: &[InboxMessage] = &[
    InboxMessage {
        sender: "Ken Bates · Chairman",
        subject: "Welcome to Chelsea FC",
        date: "18 Aug 2001",
        preview: "The board would like to welcome you to Stamford Bridge...",
        body: "Dear Manager,\n\nOn behalf of the board, welcome to Chelsea Football Club. We are delighted to have you at Stamford Bridge.\n\nOur ambition this season is to compete strongly in the Premier League and make the supporters proud. We look forward to working with you.\n\nBest wishes,\nKen Bates\nChairman",
    },
    InboxMessage {
        sender: "Steve Clarke · Assistant Manager",
        subject: "First team squad report",
        date: "18 Aug 2001",
        preview: "I've prepared a brief overview of the squad ahead of training...",
        body: "Boss,\n\nI've put together the first team squad report for your review. Our senior players are looking sharp, but we should keep an eye on fitness in the run-up to our next fixture.\n\nI'd suggest reviewing the squad and deciding on your preferred starting eleven.\n\nRegards,\nSteve Clarke\nAssistant Manager",
    },
    InboxMessage {
        sender: "Club Secretary",
        subject: "Upcoming match: Manchester United",
        date: "17 Aug 2001",
        preview: "A reminder of the forthcoming home fixture at Stamford Bridge...",
        body: "Manager,\n\nThis is a reminder that our next fixture in this demo is Chelsea FC versus Manchester United at Stamford Bridge on Saturday 25 August 2001, kick-off 15:00.\n\nPlease note that the fixture date is illustrative and is not a historical schedule record.\n\nKind regards,\nClub Secretary",
    },
];

fn text(value: impl Into<String>, color: Color32, size: f32) -> egui::Label {
    egui::Label::new(RichText::new(value).color(color).size(size))
}

fn section_heading(ui: &mut egui::Ui, heading: &str, subtitle: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(heading).color(GOLD).size(17.0).strong());
        ui.add_space(12.0);
        ui.add(text(subtitle, MUTED, 11.0));
    });
    ui.add_space(7.0);
    ui.separator();
    ui.add_space(7.0);
}

fn fake_button(ui: &mut egui::Ui, title: &str, highlighted: bool) {
    let fill = if highlighted { GOLD } else { PANEL_ALT };
    let foreground = if highlighted { HEADER } else { WHITE };
    egui::Frame::new()
        .fill(fill)
        .stroke(Stroke::new(1.0, if highlighted { GOLD } else { DIVIDER }))
        .corner_radius(2.0)
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.label(RichText::new(title).color(foreground).size(12.0).strong());
        });
}

fn boxed_panel(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(1.0, DIVIDER))
        .inner_margin(12)
        .show(ui, content);
}

fn info_line(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_sized([103.0, 18.0], text(label, MUTED, 12.0));
        ui.add(text(value, WHITE, 12.0));
    });
}

fn top_bar(root: &mut egui::Ui, screen: &mut Screen, match_state: &mut MatchState) {
    egui::Panel::top("top_bar")
        .exact_size(77.0)
        .frame(egui::Frame::new().fill(HEADER).inner_margin(egui::Margin::symmetric(18, 9)))
        .show(root, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("RETRO TOUCHLINE").color(GOLD).size(23.0).strong());
                    ui.add(text("FOOTBALL MANAGEMENT  ·  2001/02 INSPIRED", MUTED, 10.0));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let caption = if *screen == Screen::Results { "CONTINUE  »" }
                        else if match_state.finished { "FULL TIME  »" }
                        else if match_state.started { "TO THE MATCH  »" }
                        else { "CONTINUE  »" };
                    if ui.add(egui::Button::new(RichText::new(caption).color(HEADER).strong()).fill(GOLD).min_size(Vec2::new(132.0, 31.0))).clicked() {
                        if *screen == Screen::Results {
                            // Leave the completed match behind and return to the management dashboard.
                            *screen = Screen::Overview;
                            *match_state = MatchState::default();
                        } else if match_state.finished {
                            *screen = Screen::Results;
                        } else {
                            if !match_state.started { match_state.start(); }
                            *screen = Screen::Match;
                        }
                    }
                    ui.add_space(16.0);
                    ui.vertical(|ui| {
                        ui.add(text("Saturday, 18 August 2001", WHITE, 13.0));
                        ui.add(text("Pre-season  ·  15:00", MUTED, 11.0));
                    });
                });
            });
        });
}

fn side_bar(root: &mut egui::Ui, screen: &mut Screen) {
    egui::Panel::left("sidebar")
        .exact_size(192.0)
        .resizable(false)
        .frame(egui::Frame::new().fill(CHELSEA_BLUE).inner_margin(egui::Margin::symmetric(12, 16)))
        .show(root, |ui| {
            ui.label(RichText::new("CHELSEA FC").color(SIDEBAR_WHITE).size(15.0).strong());
            ui.add(text("The Blues  ·  London, England", SIDEBAR_WHITE, 11.0));
            ui.add_space(22.0);
            ui.add(text("CLUB", SIDEBAR_WHITE, 10.0));
            ui.add_space(8.0);
            for target in Screen::CLUB {
                if nav_row(ui, target.label(), *screen == target) { *screen = target; }
            }
            ui.add_space(18.0);
            ui.add(text("MANAGEMENT", SIDEBAR_WHITE, 10.0));
            ui.add_space(8.0);
            for target in Screen::MANAGEMENT {
                if nav_row(ui, target.label(), *screen == target) { *screen = target; }
            }
            ui.add_space(22.0);
            ui.separator();
            ui.add_space(10.0);
            ui.add(text("INBOX", SIDEBAR_WHITE, 10.0));
            ui.add(text("5 messages", SIDEBAR_WHITE, 12.0));
        });
}

fn nav_row(ui: &mut egui::Ui, label: &str, active: bool) -> bool {
    let fill = if active { CHELSEA_BLUE_ACTIVE } else { CHELSEA_BLUE };
    let response = egui::Frame::new()
        .fill(fill)
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_min_width(154.0);
            ui.add(egui::Button::new(
                RichText::new(if active { format!("›  {label}") } else { format!("   {label}") })
                    .color(SIDEBAR_WHITE).size(13.0)
            ).fill(fill).stroke(Stroke::NONE).min_size(egui::vec2(154.0, 20.0)))
        }).inner;
    ui.add_space(2.0);
    response.clicked()
}

// Messages for the current career. Legacy messages remain alongside the
// chairman's expectations and the manager appointment news item.
fn inbox_view(ui: &mut egui::Ui, selected_message: &mut usize, manager_name: &str) {
    let entries: Vec<(String, String, String, String, String)> = vec![
        (
            "Ken Bates · Chairman".into(), "Board expectations and transfer budget".into(),
            "18 Aug 2001".into(), "Welcome to Stamford Bridge. Your objectives for this season...".into(),
            format!("Dear {},\n\nWelcome to Chelsea Football Club. The board is pleased to appoint you as manager.\n\nOur expectations for the 2001/02 season are to compete for a European qualification place in the Premier League, remain competitive in the domestic cups, and build a disciplined, entertaining team.\n\nFor this prototype, your initial transfer budget is £2,000,000 and the illustrative weekly wage budget is £82,000. Please review your players and tactics before our upcoming match.\n\nYou will have the board's backing as you begin your career at Stamford Bridge.\n\nKen Bates\nChairman", manager_name),
        ),
        (
            "Football News · Breaking News".into(), format!("Chelsea appoint {} as new manager", manager_name),
            "18 Aug 2001".into(), "Chelsea Football Club have announced a new appointment...".into(),
            format!("CHELSEA APPOINT NEW MANAGER\n\nChelsea Football Club have confirmed the appointment of {} as their new first-team manager.\n\nThe new boss takes charge at Stamford Bridge ahead of the 2001/02 campaign. Supporters will be keen to see the new manager's tactics, squad selection and approach to transfers.\n\nThis news article is fictional content generated for your Retro Touchline career.", manager_name),
        ),
    ];
    let mut entries = entries;
    entries.extend(MESSAGES.iter().map(|m| (m.sender.into(), m.subject.into(), m.date.into(), m.preview.into(), m.body.into())));
    let count = entries.len();
    *selected_message = (*selected_message).min(count.saturating_sub(1));
    section_heading(ui, "INBOX", &format!("{} messages · Chelsea FC", count));
    ui.add(text("CLUB CORRESPONDENCE & FOOTBALL NEWS", MUTED, 11.0));
    ui.add_space(10.0);
    egui::Frame::new().fill(PANEL).stroke(Stroke::new(1.0, DIVIDER)).inner_margin(8).show(ui, |ui| {
        ui.set_width(ui.available_width());
        for (index, (sender, subject, date, preview, _)) in entries.iter().enumerate() {
            let selected = *selected_message == index;
            let fill = if selected { PANEL_ALT } else { PANEL };
            let row = egui::Frame::new().fill(fill).inner_margin(egui::Margin::symmetric(12, 10)).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(sender).color(if selected { GOLD } else { WHITE }).strong().size(13.0));
                        ui.label(RichText::new(subject).color(WHITE).size(12.0));
                        ui.add(text(preview, MUTED, 11.0));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| { ui.add(text(date, MUTED, 11.0)); });
                });
            });
            if ui.interact(row.response.rect, ui.id().with(index), egui::Sense::click()).clicked() { *selected_message = index; }
            if index + 1 != count { ui.separator(); }
        }
    });
    ui.add_space(14.0);
    boxed_panel(ui, |ui| {
        let (sender, subject, date, _, body) = &entries[*selected_message];
        ui.label(RichText::new(subject).color(GOLD).size(17.0).strong());
        ui.add_space(6.0);
        info_line(ui, "From", sender);
        info_line(ui, "Date", date);
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        ui.label(RichText::new(body).color(WHITE).size(13.0));
    });
}

// First-run club selection. Chelsea is the only selectable club at present.
fn new_career_view(ui: &mut egui::Ui, manager_name: &mut String, preview_club: &mut bool) -> bool {
    ui.vertical_centered(|ui| {
        ui.add_space(14.0);
        ui.label(RichText::new("RETRO TOUCHLINE").color(GOLD).size(31.0).strong());
        ui.label(RichText::new("NEW CAREER · 2001/02").color(MUTED).size(13.0));
        ui.add_space(20.0);
    });
    section_heading(ui, "CHOOSE YOUR CLUB", "Available clubs · England");
    boxed_panel(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("CHELSEA FC").color(GOLD).strong().size(20.0));
                ui.add(text("London · Premier League · The Blues", WHITE, 13.0));
                ui.add(text("Selected club · Chelsea is the first available team", MUTED, 11.0));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(if *preview_club { "Hide club preview" } else { "View club & squad" }).clicked() {
                    *preview_club = !*preview_club;
                }
            });
        });
        ui.add_space(8.0);
        if *preview_club {
            ui.separator();
            ui.add_space(9.0);
            ui.columns(2, |cols| {
                cols[0].label(RichText::new("CLUB INFORMATION").color(GOLD).strong());
                info_line(&mut cols[0], "Founded", "1905");
                info_line(&mut cols[0], "Ground", "Stamford Bridge");
                info_line(&mut cols[0], "City", "London, England");
                info_line(&mut cols[0], "Chairman", "Ken Bates");
                info_line(&mut cols[0], "League", "Premier League");
                info_line(&mut cols[0], "Transfer budget", "£2,000,000 (demo)");
                cols[1].label(RichText::new("2001/02 SQUAD PREVIEW").color(GOLD).strong());
                egui::ScrollArea::vertical().max_height(240.0).show(&mut cols[1], |ui| {
                    for player in PLAYERS {
                        ui.add(text(format!("#{:02}  {:<24}  {}", player.number, player.name, player.position), WHITE, 12.0));
                    }
                });
            });
        }
    });
    ui.add_space(20.0);
    section_heading(ui, "ADD MANAGER", "Set up your new managerial career");
    let mut launch = false;
    boxed_panel(ui, |ui| {
        ui.label(RichText::new("Manager name").color(WHITE).strong());
        ui.add(egui::TextEdit::singleline(manager_name).hint_text("Enter your manager's name").desired_width(340.0));
        ui.add_space(8.0);
        ui.add(text("Your name will appear in the club Inbox, appointment announcement, and Manager profile.", MUTED, 11.0));
        ui.add_space(16.0);
        let valid = !manager_name.trim().is_empty();
        let button = ui.add_enabled(valid, egui::Button::new(RichText::new("TAKE CHARGE OF CHELSEA  →").color(HEADER).strong()).fill(GOLD));
        if !valid { ui.add(text("Enter a manager name to start your career.", GOLD, 11.0)); }
        launch = button.clicked();
    });
    launch
}

// Prototype player information. Attributes and contract figures are illustrative,
// not sourced from the original Championship Manager database or real contracts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SquadTab { Squad, Attributes, Statistics, Contract }

#[derive(Clone, Copy, PartialEq, Eq)]
enum SquadStatus { FirstTeam, LoanListed, TransferListed, Released, Reserves }
impl SquadStatus {
    fn label(self) -> &'static str {
        match self {
            Self::FirstTeam => "First team", Self::LoanListed => "Available for loan",
            Self::TransferListed => "Transfer listed", Self::Released => "Released",
            Self::Reserves => "Reserves",
        }
    }
}

struct PlayerDetails {
    status: SquadStatus,
    weekly_wage: u32,
    contract_end: u16,
    appearances: u16,
    goals: u16,
    assists: u16,
    clean_sheets: u16,
    yellow_cards: u16,
    red_cards: u16,
}
impl PlayerDetails {
    fn new(player: &Player) -> Self {
        let wage = if player.position.contains("GK") { 12_000 }
            else if player.age >= 29 { 35_000 }
            else if player.age >= 23 { 22_000 } else { 6_000 };
        Self { status: SquadStatus::FirstTeam, weekly_wage: wage,
            contract_end: 2004, appearances: 0, goals: 0, assists: 0,
            clean_sheets: 0, yellow_cards: 0, red_cards: 0 }
    }
}

struct SquadEditor {
    selected: usize,
    tab: SquadTab,
    details: Vec<PlayerDetails>,
    notice: String,
}
impl Default for SquadEditor {
    fn default() -> Self {
        Self { selected: PLAYERS.iter().position(|p| p.number == 8).unwrap_or(0),
            tab: SquadTab::Squad, details: PLAYERS.iter().map(PlayerDetails::new).collect(),
            notice: String::new() }
    }
}

fn attribute_values(player: &Player) -> [(&'static str, u8); 15] {
    // Deliberately generated demo ratings, 1-20; not historical CM attributes.
    let n = player.number as u16;
    let role = if player.position.contains("GK") { 0 }
        else if player.position.contains('D') { 1 }
        else if player.position.contains('M') { 2 } else { 3 };
    let names = ["Passing", "Crossing", "Dribbling", "Finishing", "Tackling",
        "Heading", "First Touch", "Technique", "Decisions", "Determination",
        "Positioning", "Work Rate", "Pace", "Stamina", "Strength"];
    std::array::from_fn(|i| {
        let baseline: u16 = match (role, i) {
            (0, 10) | (1, 4) | (1, 10) | (2, 0) | (2, 7) | (3, 3) => 13,
            (0, 3) | (0, 2) => 3, _ => 7,
        };
        (names[i], (baseline + (n * 7 + i as u16 * 11) % 7).min(20) as u8)
    })
}

fn squad_view(ui: &mut egui::Ui, editor: &mut SquadEditor) {
    section_heading(ui, "FIRST TEAM SQUAD", "Chelsea 2001/02  ·  Select a player for details");
    ui.horizontal_wrapped(|ui| {
        for (tab, name) in [(SquadTab::Squad, "Squad"),
            (SquadTab::Attributes, "Attributes"), (SquadTab::Statistics, "Statistics"),
            (SquadTab::Contract, "Contract")] {
            if ui.selectable_label(editor.tab == tab,
                RichText::new(name).color(if editor.tab == tab { GOLD } else { WHITE })).clicked() {
                editor.tab = tab;
            }
        }
    });
    ui.add_space(10.0);
    ui.columns(2, |columns| {
        columns[0].set_min_width(390.0);
        columns[0].label(RichText::new("SQUAD ROSTER").color(GOLD).strong());
        egui::ScrollArea::vertical().id_salt("squad_roster").max_height(515.0).show(&mut columns[0], |ui| {
            egui::Grid::new("squad_select_grid").striped(true).spacing([10.0, 7.0]).show(ui, |ui| {
                for title in ["#", "Player", "Pos", "Status"] { ui.add(text(title, GOLD, 11.0)); }
                ui.end_row();
                for (idx, player) in PLAYERS.iter().enumerate() {
                    let detail = &editor.details[idx];
                    ui.add(text(player.number.to_string(), MUTED, 12.0));
                    if ui.selectable_label(editor.selected == idx,
                        RichText::new(player.name).color(WHITE).size(12.0)).clicked() {
                        editor.selected = idx;
                        editor.notice.clear();
                    }
                    ui.add(text(player.position, MUTED, 11.0));
                    ui.add(text(detail.status.label(), if detail.status == SquadStatus::FirstTeam { MUTED } else { GOLD }, 10.0));
                    ui.end_row();
                }
            });
        });
        columns[1].set_min_width(380.0);
        let player = &PLAYERS[editor.selected];
        columns[1].heading(RichText::new(format!("#{}  {}", player.number, player.name)).color(WHITE));
        columns[1].add(text(format!("{}  ·  {}  ·  Age {}", player.position, player.nationality, player.age), MUTED, 12.0));
        columns[1].add_space(8.0);
        columns[1].horizontal_wrapped(|ui| {
            ui.menu_button("Player actions ▾", |ui| {
                for (label, action) in [
                    ("New Contract", 0), ("Available for Loan", 1),
                    ("Add to Transfer Window", 2), ("Release", 3), ("Move to Reserves", 4),
                    ("Return to First Team", 5),
                ] {
                    if ui.button(label).clicked() {
                        if action == 0 { editor.tab = SquadTab::Contract;
                            editor.notice = "Contract proposal: edit wage and expiry below, then choose Offer contract.".into();
                        } else {
                            editor.details[editor.selected].status = match action {
                                1 => SquadStatus::LoanListed, 2 => SquadStatus::TransferListed,
                                3 => SquadStatus::Released, 4 => SquadStatus::Reserves,
                                _ => SquadStatus::FirstTeam,
                            };
                            editor.notice = format!("{}: {} (local demo status only).", player.name,
                                editor.details[editor.selected].status.label());
                        }
                        ui.close();
                    }
                }
            });
            columns_status(ui, editor.details[editor.selected].status.label());
        });
        if !editor.notice.is_empty() {
            columns[1].add(text(&editor.notice, GOLD, 11.0));
        }
        columns[1].add_space(10.0);
        match editor.tab {
            SquadTab::Squad => {
                info_line(&mut columns[1], "Club", "Chelsea FC");
                info_line(&mut columns[1], "Position", player.position);
                info_line(&mut columns[1], "Nationality", player.nationality);
                info_line(&mut columns[1], "Age (Aug 2001)", &player.age.to_string());
                columns[1].add_space(8.0);
                columns[1].add(text("Choose Attributes, Statistics or Contract above for detailed information.", MUTED, 12.0));
            }
            SquadTab::Attributes => {
                columns[1].add(text("Illustrative ratings (1–20), not historical game data", MUTED, 11.0));
                egui::Grid::new("attribute_grid").striped(true).spacing([16.0, 6.0]).show(&mut columns[1], |ui| {
                    for (name, value) in attribute_values(player) {
                        ui.add(text(name, WHITE, 12.0));
                        ui.add(text(value.to_string(), if value >= 15 { GOLD } else { MUTED }, 12.0));
                        ui.end_row();
                    }
                });
            }
            SquadTab::Statistics => {
                let d = &editor.details[editor.selected];
                columns[1].add(text("2001/02 season · pre-season · no matches simulated", MUTED, 11.0));
                for (name, value) in [("Appearances", d.appearances), ("Goals", d.goals),
                    ("Assists", d.assists), ("Clean sheets", d.clean_sheets),
                    ("Yellow cards", d.yellow_cards), ("Red cards", d.red_cards)] {
                    info_line(&mut columns[1], name, &value.to_string());
                }
                info_line(&mut columns[1], "Average rating", "—");
            }
            SquadTab::Contract => {
                columns[1].add(text("Illustrative contract details, not actual 2001/02 terms", MUTED, 11.0));
                let d = &mut editor.details[editor.selected];
                info_line(&mut columns[1], "Club", "Chelsea FC");
                info_line(&mut columns[1], "Status", d.status.label());
                columns[1].horizontal(|ui| { ui.label("Weekly wage (£)");
                    ui.add(egui::DragValue::new(&mut d.weekly_wage).range(0..=250_000).speed(500));
                });
                columns[1].horizontal(|ui| { ui.label("Expiry year");
                    ui.add(egui::DragValue::new(&mut d.contract_end).range(2001..=2015));
                });
                if columns[1].button("Offer contract").clicked() {
                    editor.notice = format!("Demo contract recorded for {}: £{}/week until {}.",
                        player.name, d.weekly_wage, d.contract_end);
                }
            }
        }
    });
}

fn columns_status(ui: &mut egui::Ui, status: &str) {
    ui.add(text(format!("Status: {status}"), GOLD, 11.0));
}

fn right_column(root: &mut egui::Ui) {
    egui::Panel::right("club_information")
        .exact_size(273.0)
        .resizable(false)
        .frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::symmetric(10, 14)))
        .show(root, |ui| {
            boxed_panel(ui, |ui| {
                section_heading(ui, "CLUB INFORMATION", "");
                ui.add(text("* Finances are sample UI values", MUTED, 10.0));
                info_line(ui, "Division", "Premier League");
                info_line(ui, "Stadium", "Stamford Bridge");
                info_line(ui, "2001/02 manager", "Claudio Ranieri");
                info_line(ui, "Capacity", "42,000 (demo)");
                info_line(ui, "Balance", "£4,250,000*");
                info_line(ui, "Wage budget", "£82,000 p/w*");
                info_line(ui, "Expectation", "Top six (demo)");
            });
            ui.add_space(12.0);
            boxed_panel(ui, |ui| {
                section_heading(ui, "NEXT FIXTURE", "LEAGUE");
                ui.add(text("CHELSEA FC", GOLD, 15.0));
                ui.add(text("vs", MUTED, 12.0));
                ui.add(text("MANCHESTER UNITED", WHITE, 15.0));
                ui.add_space(10.0);
                ui.add(text("Saturday 25 August 2001", WHITE, 12.0));
                ui.add(text("Stamford Bridge  ·  15:00 (demo)", MUTED, 11.0));
            });
            ui.add_space(12.0);
            boxed_panel(ui, |ui| {
                section_heading(ui, "RECENT RESULTS", "DEMO SCORES");
                for (opponent, score) in [
                    ("Arsenal", "W  3–1"),
                    ("Tottenham", "D  1–1"),
                    ("Liverpool", "W  2–0"),
                ] {
                    ui.horizontal(|ui| {
                        ui.add_sized([142.0, 22.0], text(opponent, WHITE, 12.0));
                        ui.add(text(score, GOLD, 12.0));
                    });
                }
            });
        });
}

// Data on these screens is deliberately illustrative until a game engine exists.
fn information_table(ui: &mut egui::Ui, rows: &[(&str, &str)]) {
    for (key, val) in rows { info_line(ui, key, val); ui.add_space(5.0); }
}
fn page_panel(ui: &mut egui::Ui, title: &str, subtitle: &str, rows: &[(&str, &str)]) {
    boxed_panel(ui, |ui| {
        section_heading(ui, title, subtitle);
        information_table(ui, rows);
    });
    ui.add_space(12.0);
}
fn page_view(ui: &mut egui::Ui, screen: Screen, manager_name: &str) {
    match screen {
        Screen::Overview => {
            page_panel(ui, "CLUB OVERVIEW", "Chelsea Football Club · 2001/02", &[
                ("Home", "Stamford Bridge, London"), ("Competition", "Premier League"),
                ("Manager", manager_name), ("Chairman", "Ken Bates"),
                ("Next match", "Chelsea vs Manchester United (demo fixture)"),
            ]);
            page_panel(ui, "SEASON DASHBOARD", "Not simulated", &[("League position", "—"), ("Played", "—"), ("Wins / draws / losses", "—"), ("Goals scored", "—")]);
        }
        Screen::Training => {
            page_panel(ui, "TRAINING", "Weekly training overview · mock settings", &[("Monday", "Fitness"), ("Tuesday", "Ball control"), ("Wednesday", "Tactical shape"), ("Thursday", "Attacking"), ("Friday", "Match preparation"), ("Weekend", "Rest / Matchday")]);
            page_panel(ui, "COACHING NOTES", "Display only", &[("Focus", "First-team fitness"), ("Intensity", "Normal"), ("Reports", "No reports generated")]);
        }
        Screen::Fixtures => {
            page_panel(ui, "FIXTURES & RESULTS", "Demonstration schedule · not historical fixtures", &[("25 Aug 2001", "Chelsea vs Manchester United · Home"), ("01 Sep 2001", "Arsenal vs Chelsea · Away"), ("08 Sep 2001", "Chelsea vs Liverpool · Home"), ("15 Sep 2001", "Tottenham vs Chelsea · Away")]);
            page_panel(ui, "COMPETITIONS", "Illustrative", &[("Premier League", "Upcoming fixtures"), ("FA Cup", "No draw available"), ("League Cup", "No draw available")]);
        }
        Screen::Transfers => {
            page_panel(ui, "TRANSFER CENTRE", "2001/02 arrivals · historical", &[("Frank Lampard", "West Ham United"), ("William Gallas", "Marseille"), ("Emmanuel Petit", "Barcelona"), ("Boudewijn Zenden", "Barcelona")]);
            page_panel(ui, "TRANSFER ACTIVITY", "Game systems not enabled", &[("Incoming offers", "None"), ("Outgoing offers", "None"), ("Shortlist", "No players shortlisted")]);
        }
        Screen::Scouting => {
            page_panel(ui, "SCOUTING", "Recruitment hub · demo", &[("Assignments", "No active assignments"), ("Player reports", "None available"), ("Shortlisted players", "0"), ("Region", "England (example)")]);
            page_panel(ui, "SCOUTING FILTERS", "Examples only", &[("Position", "Any"), ("Age", "Any"), ("Nationality", "Any")]);
        }
        Screen::Finances => {
            page_panel(ui, "CLUB FINANCES", "Illustrative values, not historical", &[("Bank balance", "£4,250,000 (demo)"), ("Transfer budget", "£2,000,000 (demo)"), ("Weekly wage budget", "£82,000 (demo)"), ("Weekly wage spend", "Not calculated")]);
            page_panel(ui, "FINANCIAL REPORT", "No accounting engine", &[("Matchday revenue", "—"), ("Merchandise", "—"), ("Transfer expenditure", "—")]);
        }
        Screen::Staff => {
            page_panel(ui, "CLUB STAFF", "Historical names and demo vacancies", &[("First-team manager", manager_name), ("Chairman", "Ken Bates"), ("Assistant manager", "Not assigned in prototype"), ("Head scout", "Not assigned in prototype"), ("Physio", "Not assigned in prototype")]);
            page_panel(ui, "STAFF SEARCH", "Coming later", &[("Candidates", "No staff database yet"), ("Contracts", "Not implemented")]);
        }
        Screen::Manager => {
            page_panel(ui, "MANAGER PROFILE", manager_name, &[("Club", "Chelsea FC"), ("Role", "Manager"), ("Start date", "18 August 2001 (demo)"), ("Career matches", "0"), ("Reputation", "Not calculated")]);
            page_panel(ui, "CAREER HISTORY", "Not simulated", &[("Previous clubs", "None recorded"), ("Trophies", "0"), ("Achievements", "None recorded")]);
        }
        Screen::Squad | Screen::Inbox | Screen::Tactics | Screen::LeagueTable | Screen::Match | Screen::Results => {}
    }
}


// Historically accurate 2001/02 membership, displayed with fresh-save, zeroed statistics.
// In 2001/02 the second tier was officially called the Football League First Division.
const PREMIER_LEAGUE_0102: &[&str] = &[
    "Arsenal", "Aston Villa", "Blackburn Rovers", "Bolton Wanderers",
    "Charlton Athletic", "Chelsea", "Derby County", "Everton", "Fulham",
    "Ipswich Town", "Leeds United", "Leicester City", "Liverpool",
    "Manchester United", "Middlesbrough", "Newcastle United", "Southampton",
    "Sunderland", "Tottenham Hotspur", "West Ham United",
];
const FIRST_DIVISION_0102: &[&str] = &[
    "Barnsley", "Birmingham City", "Bradford City", "Burnley",
    "Coventry City", "Crewe Alexandra", "Crystal Palace", "Gillingham",
    "Grimsby Town", "Manchester City", "Millwall", "Norwich City",
    "Nottingham Forest", "Portsmouth", "Preston North End", "Rotherham United",
    "Sheffield United", "Sheffield Wednesday", "Stockport County", "Walsall",
    "Watford", "West Bromwich Albion", "Wimbledon", "Wolverhampton Wanderers",
];

#[derive(Clone, Copy, Default)]
struct LeagueRecord {
    played: u16,
    won: u16,
    drawn: u16,
    lost: u16,
    goals_for: u16,
    goals_against: u16,
}
impl LeagueRecord {
    fn points(self) -> u16 { self.won * 3 + self.drawn }
    fn goal_difference(self) -> i32 { self.goals_for as i32 - self.goals_against as i32 }
}

fn league_standings(ui: &mut egui::Ui, clubs: &[&str]) {
    ui.label(RichText::new(format!("{} clubs · 2001/02 season", clubs.len())).color(MUTED).size(11.0));
    ui.add_space(7.0);
    egui::ScrollArea::horizontal().id_salt("competitions_table_horizontal")
        .auto_shrink([false, true]).show(ui, |ui| {
        egui::Grid::new("competitions_standings_grid")
            .striped(true).min_col_width(28.0).spacing([13.0, 9.0]).show(ui, |ui| {
            for title in ["POS", "CLUB", "P", "W", "D", "L", "GF", "GA", "GD", "PTS"] {
                ui.label(RichText::new(title).color(GOLD).strong().size(11.0));
            }
            ui.end_row();
            for (index, club) in clubs.iter().enumerate() {
                // Records are zero until match results are wired into the league engine.
                let record = LeagueRecord::default();
                let highlighted = *club == "Chelsea";
                let colour = if highlighted { GOLD } else { WHITE };
                ui.label(RichText::new((index + 1).to_string()).color(MUTED).size(12.0));
                ui.add_sized([166.0, 18.0], egui::Label::new(RichText::new(*club).color(colour).strong().size(12.0)));
                for value in [record.played.to_string(), record.won.to_string(), record.drawn.to_string(),
                    record.lost.to_string(), record.goals_for.to_string(), record.goals_against.to_string(),
                    record.goal_difference().to_string(), record.points().to_string()] {
                    ui.label(RichText::new(value).color(if highlighted { GOLD } else { WHITE }).size(12.0));
                }
                ui.end_row();
            }
        });
    });
    ui.add_space(7.0);
    ui.label(RichText::new("P: played   W: won   D: drawn   L: lost   GF: goals for   GA: goals against   GD: goal difference   PTS: points (3 per win, 1 per draw)").color(MUTED).size(10.0));
    ui.label(RichText::new("Pre-season starting table: all statistics are zero. Positions are alphabetical until games are simulated.").color(MUTED).size(10.0));
}

fn competitions_view(ui: &mut egui::Ui, tab: &mut usize, league_view: &mut usize) {
    section_heading(ui, "FOOTBALL WORLD · COMPETITIONS", "2001/02 · Browse leagues and cup competitions");
    ui.horizontal_wrapped(|ui| {
        for (index, label) in ["League Tables", "FA Cup", "Champions League", "World Overview"].iter().enumerate() {
            if ui.selectable_label(*tab == index, RichText::new(*label).color(if *tab == index { GOLD } else { WHITE }).strong()).clicked() {
                *tab = index;
            }
        }
    });
    ui.separator();
    ui.add_space(8.0);
    match *tab {
        0 => {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Competition:").color(GOLD));
                egui::ComboBox::from_id_salt("league_choice")
                    .selected_text(if *league_view == 0 { "England · Premier League" } else { "England · Championship (First Division)" })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(league_view, 0, "Premier League · 2001/02");
                        ui.selectable_value(league_view, 1, "First Division (Championship) · 2001/02");
                    });
            });
            ui.add_space(10.0);
            boxed_panel(ui, |ui| {
                if *league_view == 0 {
                    section_heading(ui, "PREMIER LEAGUE", "England · First tier · 2001/02");
                    league_standings(ui, PREMIER_LEAGUE_0102);
                } else {
                    section_heading(ui, "FIRST DIVISION / CHAMPIONSHIP", "England · Second tier · 2001/02");
                    ui.label(RichText::new("Known as the First Division in 2001/02; renamed the Championship in 2004.").color(MUTED).size(11.0));
                    league_standings(ui, FIRST_DIVISION_0102);
                }
            });
        }
        1 => {
            page_panel(ui, "FA CUP", "England · 2001/02 · Cup hub", &[
                ("Competition", "FA Challenge Cup"), ("Format", "Single-elimination knockout"),
                ("Chelsea", "Club entered · draw not simulated"), ("Fixtures", "Awaiting cup engine"),
                ("Results", "No matches simulated"),
            ]);
        }
        2 => {
            page_panel(ui, "UEFA CHAMPIONS LEAGUE", "Europe · 2001/02 · Continental hub", &[
                ("Competition", "UEFA Champions League"), ("Season", "2001/02"),
                ("Format", "Group stages and knockout rounds"), ("Chelsea", "Not a participant in 2001/02"),
                ("Fixtures", "Awaiting European competition engine"), ("Results", "No matches simulated"),
            ]);
        }
        _ => {
            page_panel(ui, "WORLD FOOTBALL", "Explore the football world · 2001/02", &[
                ("England", "Premier League · First Division (Championship)"),
                ("Domestic cups", "FA Cup"), ("Europe", "UEFA Champions League"),
                ("Other countries", "Expandable in a future database update"),
                ("Live headlines", "Not available without a match/news engine"),
            ]);
        }
    }
}

// Attack is towards the top of the screen. All positions are pitch-relative.
type Point = (f32, f32);
const SHAPE_442: [Point; 11] = [
    (0.36,0.14),(0.64,0.14),(0.13,0.39),(0.38,0.41),(0.62,0.41),(0.87,0.39),
    (0.13,0.68),(0.38,0.70),(0.62,0.70),(0.87,0.68),(0.50,0.89),
];
const INITIAL_NUMBERS: [u8;11] = [9,22,11,8,17,30,14,26,6,15,23];
const BENCH_NUMBERS: [u8;5] = [1,3,13,20,25];
const FORMATIONS: [&str; 7] = ["4-4-2", "4-3-3", "4-2-3-1", "4-1-4-1", "4-5-1", "3-5-2", "5-3-2"];
const MENTALITIES: [&str; 7] = ["Park the Bus", "Very Defensive", "Defend", "Balanced", "Positive", "Attack", "All-Out Attack"];

fn shape_for(formation: usize) -> [Point;11] {
    // Exactly eleven editable anchors: forwards, midfield, defence, goalkeeper.
    let (forwards, midfield, defence): (&[Point], &[Point], &[Point]) = match formation {
        1 => (&[(0.23,0.14),(0.50,0.11),(0.77,0.14)],
              &[(0.25,0.43),(0.50,0.46),(0.75,0.43)],
              &[(0.13,0.70),(0.38,0.70),(0.62,0.70),(0.87,0.70)]),
        2 => (&[(0.50,0.12)],
              &[(0.20,0.31),(0.50,0.28),(0.80,0.31),(0.37,0.51),(0.63,0.51)],
              &[(0.13,0.72),(0.38,0.72),(0.62,0.72),(0.87,0.72)]),
        3 => (&[(0.50,0.12)],
              &[(0.13,0.39),(0.37,0.39),(0.63,0.39),(0.87,0.39),(0.50,0.56)],
              &[(0.13,0.72),(0.38,0.72),(0.62,0.72),(0.87,0.72)]),
        4 => (&[(0.50,0.12)],
              &[(0.12,0.36),(0.31,0.44),(0.50,0.50),(0.69,0.44),(0.88,0.36)],
              &[(0.13,0.72),(0.38,0.72),(0.62,0.72),(0.87,0.72)]),
        5 => (&[(0.36,0.13),(0.64,0.13)],
              &[(0.12,0.40),(0.31,0.46),(0.50,0.49),(0.69,0.46),(0.88,0.40)],
              &[(0.24,0.71),(0.50,0.72),(0.76,0.71)]),
        6 => (&[(0.36,0.13),(0.64,0.13)],
              &[(0.25,0.44),(0.50,0.47),(0.75,0.44)],
              &[(0.09,0.62),(0.29,0.73),(0.50,0.76),(0.71,0.73),(0.91,0.62)]),
        _ => return SHAPE_442,
    };
    let mut out = [(0.5,0.5);11];
    let mut index=0;
    for point in forwards.iter().chain(midfield.iter()).chain(defence.iter()).chain([(0.5,0.89)].iter()) {
        out[index]=*point;
        index+=1;
    }
    out
}
fn player_by_number(number: u8) -> usize {
    PLAYERS.iter().position(|p| p.number == number).expect("historical player")
}
fn initial_eleven() -> [Option<usize>;11] { INITIAL_NUMBERS.map(|n| Some(player_by_number(n))) }
fn initial_bench() -> [Option<usize>;5] { BENCH_NUMBERS.map(|n| Some(player_by_number(n))) }
fn short_name(name: &str) -> String { name.split_whitespace().last().unwrap_or(name).to_owned() }

#[derive(Clone)]
struct Tactic {
    name: String,
    formation: usize,
    mentality: usize,
    xi: [Option<usize>;11],
    bench: [Option<usize>;5],
    locations: [Point;11],
}
impl Tactic {
    fn default_chelsea() -> Self {
        Self { name:"Chelsea Classic 4-4-2".into(), formation:0, mentality:3,
            xi:initial_eleven(), bench:initial_bench(), locations:SHAPE_442 }
    }
    fn put_on_pitch(&mut self, slot:usize, player:usize) {
        let old_xi=self.xi.iter().position(|p| *p==Some(player));
        let old_bench=self.bench.iter().position(|p| *p==Some(player));
        let replaced=self.xi[slot];
        self.xi[slot]=Some(player);
        if let Some(source)=old_xi { if source!=slot {self.xi[source]=replaced;} }
        if let Some(source)=old_bench { self.bench[source]=replaced; }
    }
    fn put_on_bench(&mut self, slot:usize, player:usize) {
        let old_xi=self.xi.iter().position(|p| *p==Some(player));
        let old_bench=self.bench.iter().position(|p| *p==Some(player));
        let replaced=self.bench[slot];
        self.bench[slot]=Some(player);
        if let Some(source)=old_xi { self.xi[source]=replaced; }
        if let Some(source)=old_bench { if source!=slot {self.bench[source]=replaced;} }
    }
    fn change_formation(&mut self, id:usize) { self.formation=id; self.locations=shape_for(id); }
}

struct TacticsEditor {
    current:Tactic,
    saved: Vec<Tactic>,
    selected: Option<usize>,
    drag_player:Option<usize>,
    chosen_preset:usize,
    notice:String,
}
impl Default for TacticsEditor {
    fn default()->Self { Self { current:Tactic::default_chelsea(), saved:vec![], selected:None,
        drag_player:None, chosen_preset:0, notice:"Adjust and save a named tactic for this session".into() } }
}
fn shirt(ui:&egui::Ui, center:Pos2, number:u8, highlighted:bool) {
    let pts=[(-11.,-16.),(-21.,-10.),(-26.,2.),(-16.,7.),(-12.,2.),(-12.,19.),
        (12.,19.),(12.,2.),(16.,7.),(26.,2.),(21.,-10.),(11.,-16.),(6.,-12.),(-6.,-12.)];
    let points=pts.into_iter().map(|(x,y)|center+Vec2::new(x,y)).collect();
    ui.painter().add(egui::Shape::convex_polygon(points,
        if highlighted {CHELSEA_BLUE_ACTIVE}else{CHELSEA_BLUE},Stroke::new(1.8,Color32::WHITE)));
    ui.painter().text(center+Vec2::new(0.,1.),Align2::CENTER_CENTER,number.to_string(),
        egui::FontId::proportional(16.),Color32::WHITE);
}
fn pitch_view(ui:&mut egui::Ui, editor:&mut TacticsEditor)->Rect {
    let width=ui.available_width().clamp(260.,430.);
    let height=510.;
    let (rect, response)=ui.allocate_exact_size(Vec2::new(width,height),Sense::click_and_drag());
    let painter=ui.painter_at(rect);
    painter.rect_filled(rect,4.,Color32::from_rgb(25,105,67));
    for band in 0..10 {
        if band%2==0 {
            let y1=rect.top()+height*band as f32/10.;
            let y2=rect.top()+height*(band+1) as f32/10.;
            painter.rect_filled(Rect::from_min_max(Pos2::new(rect.left(),y1),Pos2::new(rect.right(),y2)),0.,
                Color32::from_rgba_unmultiplied(255,255,255,12));
        }
    }
    let line=Stroke::new(1.5,Color32::from_rgb(199,234,210));
    let field=rect.shrink2(Vec2::new(13.,13.));
    for (a,b) in [(field.left_top(),field.right_top()),(field.right_top(),field.right_bottom()),
        (field.right_bottom(),field.left_bottom()),(field.left_bottom(),field.left_top()),
        (field.left_center(),field.right_center())] {painter.line_segment([a,b],line);}
    painter.circle_stroke(field.center(),42.,line);
    let cx=field.center().x;
    let half=field.width()*0.29;
    for top in [true,false] {
        let y=if top {field.top()}else{field.bottom()};
        let sign=if top {1.}else{-1.};
        let end=y+sign*74.;
        for (a,b) in [(Pos2::new(cx-half,y),Pos2::new(cx-half,end)),
            (Pos2::new(cx-half,end),Pos2::new(cx+half,end)),
            (Pos2::new(cx+half,end),Pos2::new(cx+half,y))] {painter.line_segment([a,b],line);}
    }
    for slot in 0..11 {
        let (x,y)=editor.current.locations[slot];
        let center=Pos2::new(rect.left()+x*width,rect.top()+y*height);
        let hit=Rect::from_center_size(center,Vec2::new(59.,66.));
        let resp=ui.interact(hit,ui.id().with(("position",slot)),Sense::click_and_drag());
        if resp.drag_started() {
            if let Some(player)=editor.current.xi[slot] {
                editor.drag_player=Some(player); editor.selected=Some(player);
            }
        }
        if resp.clicked() {
            if let Some(player)=editor.selected {
                editor.current.put_on_pitch(slot,player);editor.selected=None;
            } else {editor.selected=editor.current.xi[slot];}
        }
        if resp.secondary_clicked() {editor.current.xi[slot]=None;editor.selected=None;}
        if let Some(index)=editor.current.xi[slot] {
            let player=&PLAYERS[index];
            shirt(ui,center,player.number,editor.selected==Some(index));
            ui.painter().text(center+Vec2::new(0.,30.),Align2::CENTER_CENTER,short_name(player.name),
                egui::FontId::proportional(10.),Color32::WHITE);
        } else {
            ui.painter().circle_stroke(center,17.,Stroke::new(1.4,WHITE));
            ui.painter().text(center,Align2::CENTER_CENTER,"+",egui::FontId::proportional(21.),WHITE);
        }
    }
    // Clicking empty grass with a selected squad player fills the closest starting-XI anchor.
    if response.clicked() {
        if let (Some(player),Some(pos))=(editor.selected,response.interact_pointer_pos()) {
            if let Some(slot)=nearest_slot(rect,&editor.current.locations,pos) {
                editor.current.put_on_pitch(slot,player);
                editor.selected=None;
            }
        }
    }
    rect
}
fn nearest_slot(rect:Rect,points:&[Point;11],pos:Pos2)->Option<usize> {
    if !rect.contains(pos) {return None;}
    points.iter().enumerate().map(|(index,&(x,y))| {
        let pixel=Pos2::new(rect.left()+x*rect.width(),rect.top()+y*rect.height());
        (index,pixel.distance_sq(pos))
    }).min_by(|a,b|a.1.total_cmp(&b.1)).map(|(index,_)|index)
}
fn drop_on_pitch(editor:&mut TacticsEditor,rect:Rect,pos:Pos2,player:usize) {
    if let Some(slot)=editor.current.xi.iter().position(|p|*p==Some(player)) {
        // Existing starter: completely free positioning, including custom asymmetric shapes.
        let x=((pos.x-rect.left())/rect.width()).clamp(0.07,0.93);
        let y=((pos.y-rect.top())/rect.height()).clamp(0.07,0.93);
        editor.current.locations[slot]=(x,y);
        editor.notice="Custom on-pitch position updated".into();
    } else if let Some(slot)=nearest_slot(rect,&editor.current.locations,pos) {
        editor.current.put_on_pitch(slot,player);
        editor.notice="Starting XI updated".into();
    }
}
fn bench_view(ui:&mut egui::Ui,editor:&mut TacticsEditor)->Vec<Rect> {
    let mut rects=Vec::new();
    ui.label(RichText::new("SUBSTITUTES · 5").color(GOLD).size(12.).strong());
    for slot in 0..5 {
        let occupant=editor.current.bench[slot];
        let line=egui::Frame::new().fill(PANEL_ALT).inner_margin(egui::Margin::symmetric(8,5))
            .show(ui,|ui| {
                ui.set_min_width(190.);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("S{}",slot+1)).color(GOLD).strong());
                    match occupant {
                        Some(p)=>{ui.label(RichText::new(format!("#{}  {}",PLAYERS[p].number,PLAYERS[p].name)).color(WHITE).size(11.));},
                        None=>{ui.label(RichText::new("+ Empty substitute").color(MUTED).size(11.));}
                    }
                });
            });
        let r=ui.interact(line.response.rect,ui.id().with(("bench",slot)),Sense::click_and_drag());
        if r.clicked() {
            if let Some(player)=editor.selected {editor.current.put_on_bench(slot,player);editor.selected=None;}
            else {editor.selected=occupant;}
        }
        if r.secondary_clicked() {editor.current.bench[slot]=None;editor.selected=None;}
        if r.drag_started() {editor.drag_player=occupant;editor.selected=occupant;}
        rects.push(line.response.rect);
        ui.add_space(3.);
    }
    rects
}
fn roster_view(ui:&mut egui::Ui,editor:&mut TacticsEditor) {
    ui.label(RichText::new("CHELSEA SQUAD · 2001/02").color(GOLD).size(12.).strong());
    egui::ScrollArea::vertical().id_salt("tactics_roster_scroll").max_height(325.).show(ui,|ui| {
        for (index,player) in PLAYERS.iter().enumerate() {
            let starter=editor.current.xi.contains(&Some(index));
            let sub=editor.current.bench.contains(&Some(index));
            let active=editor.selected==Some(index);
            let row=egui::Frame::new()
                .fill(if active {CHELSEA_BLUE_ACTIVE}else if starter||sub {PANEL_ALT}else{PANEL})
                .inner_margin(egui::Margin::symmetric(7,4)).show(ui,|ui|{
                    ui.set_min_width(195.);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{:>2}",player.number)).color(GOLD).size(11.).strong());
                        ui.label(RichText::new(format!("{}  {}",player.name,
                            if starter {"(XI)"}else if sub {"(SUB)"}else{""})).color(WHITE).size(11.));
                    });
                });
            let r=ui.interact(row.response.rect,ui.id().with(("roster",index)),Sense::click_and_drag());
            if r.clicked() {editor.selected=Some(index);}
            if r.drag_started() {editor.drag_player=Some(index);editor.selected=Some(index);}
            ui.add_space(2.);
        }
    });
}
fn tactics_view(ui:&mut egui::Ui,editor:&mut TacticsEditor) {
    section_heading(ui,"TACTICS","Chelsea FC · XI + five substitutes · custom tactics");
    ui.horizontal_wrapped(|ui| {
        egui::ComboBox::from_label("Formation")
            .selected_text(FORMATIONS[editor.current.formation]).show_ui(ui,|ui|{
                for (id,label) in FORMATIONS.iter().enumerate() {
                    if ui.selectable_label(editor.current.formation==id,*label).clicked() {
                        editor.current.change_formation(id);
                        editor.notice=format!("{} applied; starting XI retained",label);
                    }
                }
            });
        egui::ComboBox::from_label("Mentality")
            .selected_text(MENTALITIES[editor.current.mentality]).show_ui(ui,|ui|{
                for (id,label) in MENTALITIES.iter().enumerate() {
                    ui.selectable_value(&mut editor.current.mentality,id,*label);
                }
            });
    });
    ui.add_space(5.);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Tactic name:").color(GOLD));
        ui.add_sized([210.,24.],egui::TextEdit::singleline(&mut editor.current.name));
        if ui.button("Save tactic").clicked() {
            let name=editor.current.name.trim().to_owned();
            if name.is_empty() {editor.notice="Enter a tactic name before saving".into();}
            else {
                editor.current.name=name.clone();
                if let Some(index)=editor.saved.iter().position(|t|t.name==name) {
                    editor.saved[index]=editor.current.clone();editor.chosen_preset=index;
                } else {
                    editor.saved.push(editor.current.clone());editor.chosen_preset=editor.saved.len()-1;
                }
                editor.notice=format!("Saved '{}' for this session",name);
            }
        }
        if !editor.saved.is_empty() {
            let selected_name=editor.saved.get(editor.chosen_preset).map(|t|t.name.as_str()).unwrap_or("Choose tactic");
            egui::ComboBox::from_id_salt("saved_tactics").selected_text(selected_name).show_ui(ui,|ui|{
                for (id,tactic) in editor.saved.iter().enumerate() {
                    ui.selectable_value(&mut editor.chosen_preset,id,&tactic.name);
                }
            });
            if ui.button("Load").clicked() {
                if let Some(tactic)=editor.saved.get(editor.chosen_preset) {
                    editor.current=tactic.clone();editor.selected=None;
                    editor.notice="Saved tactic loaded".into();
                }
            }
        }
    });
    ui.horizontal_wrapped(|ui| {
        if ui.button("Reset shape").clicked() {editor.current.locations=shape_for(editor.current.formation);}
        if ui.button("Reset Chelsea XI + bench").clicked() {
            editor.current.xi=initial_eleven();editor.current.bench=initial_bench();editor.selected=None;
        }
        if ui.button("Clear XI").clicked() {editor.current.xi=[None;11];editor.selected=None;}
        if ui.button("Clear bench").clicked() {editor.current.bench=[None;5];editor.selected=None;}
        ui.label(RichText::new(format!("XI {}/11 · Bench {}/5",
            editor.current.xi.iter().flatten().count(),editor.current.bench.iter().flatten().count())).color(GOLD));
    });
    ui.label(RichText::new("Select a player then click a pitch slot or substitute slot. Drag a starter anywhere on the pitch to create a custom shape. Drag squad players to the pitch or bench. Right-click to clear a slot.").color(MUTED).size(11.));
    ui.add_space(8.);
    let mut pitch_rect=Rect::NOTHING;
    let mut bench_rects=Vec::new();
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new(format!("STARTING XI · {}",FORMATIONS[editor.current.formation])).color(GOLD).strong());
            ui.add_space(5.);
            pitch_rect=pitch_view(ui,editor);
        });
        ui.add_space(9.);
        ui.vertical(|ui| {
            ui.set_min_width(205.);
            bench_rects=bench_view(ui,editor);
            ui.add_space(12.);
            roster_view(ui,editor);
        });
    });
    if ui.ctx().input(|i|i.pointer.any_released()) {
        if let Some(player)=editor.drag_player.take() {
            if let Some(pos)=ui.ctx().input(|i|i.pointer.latest_pos()) {
                if pitch_rect.contains(pos) {drop_on_pitch(editor,pitch_rect,pos,player);editor.selected=None;}
                else if let Some(index)=bench_rects.iter().position(|r|r.contains(pos)) {
                    editor.current.put_on_bench(index,player);editor.selected=None;
                    editor.notice="Substitutes bench updated".into();
                }
            }
        }
    }
    if let Some(player)=editor.drag_player {
        if let Some(pos)=ui.ctx().input(|i|i.pointer.hover_pos()) {
            ui.painter().text(pos+Vec2::new(12.,-10.),Align2::LEFT_CENTER,
                format!("#{} {}",PLAYERS[player].number,PLAYERS[player].name),egui::FontId::proportional(12.),GOLD);
        }
        ui.ctx().request_repaint();
    }
    ui.add_space(7.);
    ui.label(RichText::new(&editor.notice).color(MUTED).size(11.));
    ui.label(RichText::new("Tactics and saved presets are held in memory only; no match engine or disk persistence yet.").color(MUTED).size(10.));
}


// MATCH CENTRE: deterministic scripted demonstration. No results are written to league tables.
#[derive(Clone, Copy)]
struct MatchEvent {
    at: u32, // elapsed match seconds
    side: usize, // 0 Chelsea, 1 Manchester United
    kind: &'static str,
    heading: &'static str,
    commentary: &'static str,
    chelsea_goal: bool,
    united_goal: bool,
}
const MATCH_EVENTS: &[MatchEvent] = &[
    MatchEvent { at: 60, side: 0, kind: "KICK OFF", heading: "Match begins", commentary: "We're underway at Stamford Bridge! Chelsea kick off against Manchester United.", chelsea_goal: false, united_goal: false },
    MatchEvent { at: 5*60, side: 0, kind: "CHANCE", heading: "Lampard shoots", commentary: "Lampard finds space 25 yards out, but his shot is held comfortably by Barthez.", chelsea_goal: false, united_goal: false },
    MatchEvent { at: 13*60, side: 0, kind: "GOAL", heading: "Hasselbaink (13')", commentary: "GOAL! Hasselbaink drives Zenden's cross into the bottom corner! Chelsea lead!", chelsea_goal: true, united_goal: false },
    MatchEvent { at: 24*60, side: 1, kind: "YELLOW", heading: "Roy Keane (24')", commentary: "Roy Keane is booked for a late challenge on Frank Lampard.", chelsea_goal: false, united_goal: false },
    MatchEvent { at: 37*60, side: 1, kind: "GOAL", heading: "van Nistelrooy (37')", commentary: "GOAL! Van Nistelrooy equalises from close range for Manchester United.", chelsea_goal: false, united_goal: true },
    MatchEvent { at: 43*60, side: 0, kind: "CHANCE", heading: "Gudjohnsen close", commentary: "Gudjohnsen curls one narrowly wide. The home supporters thought that was in!", chelsea_goal: false, united_goal: false },
    MatchEvent { at: 50*60, side: 1, kind: "CHANCE", heading: "Giggs breaks free", commentary: "Giggs accelerates down the left, but Cudicini gathers the low cross.", chelsea_goal: false, united_goal: false },
    MatchEvent { at: 59*60, side: 0, kind: "SUB", heading: "Zola on / Zenden off (59')", commentary: "Chelsea make a change: Gianfranco Zola replaces Boudewijn Zenden.", chelsea_goal: false, united_goal: false },
    MatchEvent { at: 71*60, side: 0, kind: "GOAL", heading: "Lampard (71')", commentary: "GOAL! Frank Lampard meets Zola's layoff with a fierce strike! 2–1 Chelsea!", chelsea_goal: true, united_goal: false },
    MatchEvent { at: 79*60, side: 1, kind: "RED", heading: "Roy Keane (79')", commentary: "RED CARD! Roy Keane receives a second yellow and Manchester United are down to ten men!", chelsea_goal: false, united_goal: false },
    MatchEvent { at: 86*60, side: 1, kind: "SUB", heading: "Solskjaer on / Cole off (86')", commentary: "Manchester United send on Ole Gunnar Solskjaer for Andy Cole.", chelsea_goal: false, united_goal: false },
    MatchEvent { at: 90*60+90, side: 0, kind: "CHANCE", heading: "Chelsea attack (90+2')", commentary: "Chelsea push for a third in stoppage time, but the ball is cleared to safety.", chelsea_goal: false, united_goal: false },
];
struct MatchState {
    started: bool, paused: bool, halftime: bool, halftime_pending: bool, finished: bool,
    elapsed: f32, last_tick: Instant, speed: f32, processed: usize,
    home_goals: u8, away_goals: u8, commentary: Vec<String>,
}
impl Default for MatchState {
    fn default() -> Self { Self { started:false,paused:true,halftime:false,halftime_pending:false,finished:false,
        elapsed:0.0,last_tick:Instant::now(),speed:4.0,processed:0,home_goals:0,away_goals:0,commentary:vec!["Welcome to Stamford Bridge. Chelsea v Manchester United — demonstration fixture.".into()] } }
}
impl MatchState {
    fn start(&mut self) { *self=Self::default(); self.started=true; self.paused=false; self.last_tick=Instant::now(); }
    fn resume(&mut self) { self.paused=false; self.halftime=false; self.last_tick=Instant::now(); }
    fn clock(&self)->String {
        let seconds=self.elapsed.floor() as u32;
        if seconds >= 90*60 { format!("90+{}:{:02}", (seconds-90*60)/60, seconds%60) }
        else { format!("{:02}:{:02}",seconds/60,seconds%60) }
    }
    fn tick(&mut self) {
        let now=Instant::now(); let delta=now.duration_since(self.last_tick).as_secs_f32().min(0.25);self.last_tick=now;
        if !self.started || self.paused || self.finished {return;}
        let old=self.elapsed;
        self.elapsed=(self.elapsed+delta*60.0*self.speed).min(94.0*60.0);
        if !self.halftime && old < 45.0*60.0 && self.elapsed >= 45.0*60.0 {
            self.elapsed=45.0*60.0; self.halftime=true; self.paused=true; self.halftime_pending=true;
            self.commentary.push("HALF TIME — Chelsea 1, Manchester United 1. Review your tactics and start the second half.".into());
        }
        while self.processed < MATCH_EVENTS.len() && MATCH_EVENTS[self.processed].at as f32 <= self.elapsed {
            let event=MATCH_EVENTS[self.processed];
            self.home_goals+=u8::from(event.chelsea_goal);self.away_goals+=u8::from(event.united_goal);
            self.commentary.push(format!("{:02}'  {}", event.at/60,event.commentary));
            self.processed+=1;
        }
        if self.elapsed >= 94.0*60.0 {
            self.finished=true;self.paused=true;
            self.commentary.push(format!("FULL TIME — Chelsea {}–{} Manchester United. This was a scripted demonstration.",self.home_goals,self.away_goals));
        }
    }
}
fn match_event_list(ui: &mut egui::Ui, side: usize, elapsed: f32) {
    for event in MATCH_EVENTS.iter().filter(|event|event.side==side && (event.at as f32)<=elapsed && matches!(event.kind,"GOAL"|"RED"|"YELLOW"|"SUB")) {
        let color=match event.kind {"GOAL"=>GOLD,"RED"=>Color32::from_rgb(248,102,98),"YELLOW"=>Color32::from_rgb(247,213,90),_=>MUTED};
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(event.kind).color(color).size(10.0).strong());
            ui.label(RichText::new(event.heading).color(WHITE).size(11.0));
        });ui.add_space(4.0);
    }
}
fn match_view(ui: &mut egui::Ui, state: &mut MatchState, screen: &mut Screen) {
    section_heading(ui,"MATCH CENTRE","Chelsea FC  v  Manchester United  •  Premier League demo");
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(if state.finished {"FULL TIME"} else if state.halftime {"HALF TIME"} else if state.paused {"PAUSED"} else {"LIVE SIMULATION"}).color(GOLD).strong());
        ui.separator();
        ui.label(RichText::new(format!("MATCH CLOCK  {}",state.clock())).color(WHITE).size(18.0).strong());
        ui.separator();
        if !state.finished {
            if ui.button(if state.paused {"▶ Resume"} else {"Ⅱ Pause"}).clicked() {
                if state.paused {state.resume();} else {state.paused=true;}
            }
            if ui.button("⚙ Tactics & substitutions").clicked() {state.paused=true;*screen=Screen::Tactics;}
            egui::ComboBox::from_id_salt("match_speed").selected_text(format!("{}x speed",state.speed as u32))
                .show_ui(ui,|ui| { for speed in [1.0,2.0,4.0,8.0,16.0] {ui.selectable_value(&mut state.speed,speed,format!("{}x",speed as u32));} });
        } else {
            if ui.add(egui::Button::new(RichText::new("FULL TIME  →  ALL RESULTS").color(HEADER).strong()).fill(GOLD)).clicked() {
                *screen = Screen::Results;
            }
            if ui.button("↻ Replay demo").clicked() {state.start();}
        }
    });
    ui.add_space(10.0);
    boxed_panel(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("CHELSEA FC").color(GOLD).size(21.0).strong());
                ui.add(text("Stamford Bridge  •  Home", MUTED, 11.0));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center),|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("MANCHESTER UNITED").color(WHITE).size(21.0).strong());
                    ui.add(text("Away",MUTED,11.0));
                });
            });
        });
        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(format!("{}    —    {}",state.home_goals,state.away_goals)).color(GOLD).size(44.0).strong());
            ui.label(RichText::new(if state.finished {"Full Time"}else if state.halftime {"Half Time — change tactics"}else{"2001/02  •  Exhibition match"}).color(MUTED).size(12.0));
        });
        ui.add_space(12.0);
        ui.columns(2,|columns| {
            columns[0].label(RichText::new("CHELSEA — MATCH EVENTS").color(GOLD).strong());
            columns[1].label(RichText::new("MANCHESTER UNITED — MATCH EVENTS").color(GOLD).strong());
            columns[0].separator(); columns[1].separator();
            match_event_list(&mut columns[0],0,state.elapsed);
            match_event_list(&mut columns[1],1,state.elapsed);
            for column in columns.iter_mut() {column.add_space(110.0);}
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(RichText::new("POSSESSION").color(MUTED).strong());
            let home=(52.0 + (state.elapsed/240.0).sin()*4.0) / 100.0;
            ui.add(egui::ProgressBar::new(home).desired_width(ui.available_width().min(620.0)).text(format!("Chelsea {:.0}%   —   United {:.0}%",home*100.0,(1.0-home)*100.0)));
        });
    });
    ui.add_space(12.0);
    boxed_panel(ui,|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("LIVE MATCH COMMENTARY").color(GOLD).strong());
            ui.add(text("Scripted events • newest first",MUTED,11.0));
        });
        ui.separator();
        egui::ScrollArea::vertical().id_salt("live_commentary").max_height(165.0).show(ui,|ui| {
            for message in state.commentary.iter().rev().take(12) {
                ui.label(RichText::new(message).color(if message.contains("GOAL!") { GOLD } else { WHITE }).size(12.0));
                ui.add_space(4.0);
            }
        });
    });
    ui.add_space(8.0);
    ui.label(RichText::new("Demo only: timeline, events and possession are simulated. Tactics changes do not yet affect the result. Score is not written to the league table.").color(MUTED).size(10.0));
}

// A complete 2001/02 Premier League matchday. Opponents are historically valid
// for the division; pairings and scores are invented demonstration data, NOT
// the real fixtures/results for 25 August 2001.
struct DemoFixture {
    home: &'static str,
    away: &'static str,
    home_score: u8,
    away_score: u8,
}
const DEMO_MATCHDAY: &[DemoFixture] = &[
    DemoFixture {home:"Chelsea",away:"Manchester United",home_score:0,away_score:0}, // live match replaces score
    DemoFixture {home:"Arsenal",away:"Leeds United",home_score:2,away_score:1},
    DemoFixture {home:"Liverpool",away:"West Ham United",home_score:3,away_score:0},
    DemoFixture {home:"Newcastle United",away:"Tottenham Hotspur",home_score:1,away_score:1},
    DemoFixture {home:"Blackburn Rovers",away:"Bolton Wanderers",home_score:0,away_score:2},
    DemoFixture {home:"Aston Villa",away:"Everton",home_score:1,away_score:0},
    DemoFixture {home:"Charlton Athletic",away:"Derby County",home_score:2,away_score:2},
    DemoFixture {home:"Fulham",away:"Ipswich Town",home_score:1,away_score:2},
    DemoFixture {home:"Leicester City",away:"Middlesbrough",home_score:0,away_score:0},
    DemoFixture {home:"Southampton",away:"Sunderland",home_score:2,away_score:1},
];

fn matchday_results_view(ui: &mut egui::Ui, match_state: &mut MatchState, screen: &mut Screen) {
    section_heading(ui, "PREMIER LEAGUE · RESULTS", "2001/02 · Demonstration matchday · Saturday 25 August 2001");
    boxed_panel(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("FULL TIME").color(GOLD).strong().size(15.0));
            ui.separator();
            ui.label(RichText::new("10 matches · 20 clubs").color(WHITE));
            ui.separator();
            ui.label(RichText::new("England · Premier League").color(MUTED));
        });
        ui.add_space(10.0);
        egui::Grid::new("matchday_results_grid")
            .num_columns(4).striped(true).spacing([18.0, 12.0]).show(ui, |ui| {
            ui.label(RichText::new("HOME").color(GOLD).strong());
            ui.label(RichText::new("FT").color(GOLD).strong());
            ui.label(RichText::new("AWAY").color(GOLD).strong());
            ui.label(RichText::new("STATUS").color(GOLD).strong());
            ui.end_row();
            for (index, match_) in DEMO_MATCHDAY.iter().enumerate() {
                let selected = index == 0;
                let (home, away) = if selected {
                    (match_state.home_goals, match_state.away_goals)
                } else {
                    (match_.home_score, match_.away_score)
                };
                let color = if selected { GOLD } else { WHITE };
                ui.label(RichText::new(match_.home).color(color).strong());
                ui.label(RichText::new(format!("{} - {}", home, away)).color(color).strong().size(15.0));
                ui.label(RichText::new(match_.away).color(color).strong());
                ui.label(RichText::new(if selected { "YOUR MATCH · FT" } else { "FT" }).color(if selected { GOLD } else { MUTED }).size(11.0));
                ui.end_row();
            }
        });
        ui.add_space(12.0);
        ui.label(RichText::new("Chelsea's score comes from the Match Centre. All other scores and pairings are illustrative, not historical 2001/02 results. League standings are not yet updated.").color(MUTED).size(11.0));
    });
    ui.add_space(14.0);
    ui.horizontal(|ui| {
        if ui.add(egui::Button::new(RichText::new("CONTINUE  →  MANAGEMENT").color(HEADER).strong()).fill(GOLD)).clicked() {
            *screen = Screen::Overview;
            *match_state = MatchState::default();
        }
        ui.add_space(10.0);
        ui.label(RichText::new("Return to your Chelsea dashboard to manage Squad, Tactics, Transfers and more.").color(MUTED).size(11.0));
    });
}

fn bottom_bar(root: &mut egui::Ui, screen: Screen) {
    egui::Panel::bottom("status")
        .exact_size(34.0)
        .frame(egui::Frame::new().fill(HEADER).inner_margin(egui::Margin::symmetric(16, 7)))
        .show(root, |ui| {
            ui.horizontal(|ui| {
                ui.add(text(format!("CHELSEA FC  |  {}", screen.label()), GOLD, 11.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(text("2001/02  •  SIMULATED MATCH PREVIEW", MUTED, 10.0));
                });
            });
        });
}

struct FootballManagerUi {
    screen: Screen,
    selected_message: usize,
    tactics: TacticsEditor,
    competition_tab: usize,
    league_view: usize,
    squad: SquadEditor,
    match_state: MatchState,
    setup_complete: bool,
    manager_name: String,
    preview_club: bool,
}

impl Default for FootballManagerUi {
    fn default() -> Self {
        Self { screen: Screen::Squad, selected_message: 0, tactics: TacticsEditor::default(), competition_tab: 0, league_view: 0, squad: SquadEditor::default(), match_state: MatchState::default(), setup_complete: false, manager_name: String::new(), preview_club: true }
    }
}

impl eframe::App for FootballManagerUi {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        root.ctx().set_visuals(egui::Visuals::dark());
        if !self.setup_complete {
            egui::CentralPanel::default()
                .frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::symmetric(32, 24)))
                .show(root, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if new_career_view(ui, &mut self.manager_name, &mut self.preview_club) {
                            self.manager_name = self.manager_name.trim().to_owned();
                            self.setup_complete = true;
                            self.selected_message = 0;
                            self.screen = Screen::Inbox;
                        }
                    });
                });
            return;
        }
        self.match_state.tick();
        if self.match_state.halftime_pending {
            self.match_state.halftime_pending = false;
            self.screen = Screen::Tactics;
        }
        top_bar(root, &mut self.screen, &mut self.match_state);
        if !matches!(self.screen, Screen::Match | Screen::Results) { side_bar(root, &mut self.screen); }
        bottom_bar(root, self.screen);
        if !matches!(self.screen, Screen::Match | Screen::Results) { right_column(root); }
        if self.match_state.started && !self.match_state.paused && !self.match_state.finished {
            root.ctx().request_repaint_after(std::time::Duration::from_millis(80));
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::symmetric(17, 16)))
            .show(root, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(text(format!("Chelsea FC  /  {}", self.screen.label()), MUTED, 11.0));
                    ui.add_space(15.0);
                    if self.screen == Screen::Tactics && self.match_state.started && !self.match_state.finished {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(if self.match_state.halftime { "HALF-TIME: 45:00 — Adjust your tactics" } else { "MATCH PAUSED — Adjust your tactics" }).color(GOLD).strong());
                            if ui.button(if self.match_state.halftime { "START SECOND HALF  ▶" } else { "RESUME MATCH  ▶" }).clicked() {
                                self.match_state.resume();
                                self.screen = Screen::Match;
                            }
                        });
                        ui.add_space(10.0);
                    }
                    match self.screen {
                        Screen::Match => match_view(ui, &mut self.match_state, &mut self.screen),
                        Screen::Results => matchday_results_view(ui, &mut self.match_state, &mut self.screen),
                        Screen::Squad => boxed_panel(ui, |ui| squad_view(ui, &mut self.squad)),
                        Screen::Inbox => inbox_view(ui, &mut self.selected_message, &self.manager_name),
                        Screen::Tactics => tactics_view(ui, &mut self.tactics),
                        Screen::LeagueTable => competitions_view(ui, &mut self.competition_tab, &mut self.league_view),
                        other => page_view(ui, other, &self.manager_name),
                    }
                });
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("Retro Touchline").with_inner_size([1280.0, 780.0]).with_min_inner_size([1050.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native("Retro Touchline", options, Box::new(|_cc| Ok(Box::new(FootballManagerUi::default()))))
}
