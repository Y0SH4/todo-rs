use ncurses::*;
use std::cmp::*;

const REGULAR_PAIR: i16 = 1;
const HIGHLIGHT_PAIR: i16 = 2;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id, prefix: &str) {
        assert!(self.list_curr.is_none(), "Nested lists are not allowed!");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) {
        let id_curr = self
            .list_curr
            .expect("Not allowed to create list elements outside of lists");
        self.label(&format!("- [ ] {}", label), {
            if id_curr == id {
                HIGHLIGHT_PAIR
            }else {
                REGULAR_PAIR
            }
        })
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    fn end_list(&mut self) {
        self.list_curr = None;
    }

    fn end(&mut self) {
        // Add any cleanup or finalization logic if needed
    }
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut todos:Vec<String> = vec![
        "Learning javascript".to_string(),
        "Playing dota2 with friends".to_string(),
        "Cooking some burger".to_string(),
    ];

    let mut todo_curr: usize = 0;
    let dones: Vec<String> = vec![
        "Play with baby".to_string(),
        "Learn guitar scale pentatonic".to_string()
    ];
    let mut done_curr: usize = 0;

    let mut ui = Ui::default();
    while !quit {
        ui.begin(0, 0);
        {
            ui.label("TODO:", REGULAR_PAIR);
            ui.begin_list(todo_curr);
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(todo, index);
            }
            ui.end_list();
            ui.label("--------------------------------------------------------", REGULAR_PAIR);
            ui.label("DONE:", REGULAR_PAIR);
            ui.begin_list(done_curr + 6969);
            for (index, done) in dones.iter().enumerate(){
                ui.list_element(done,index + 6969);
            }
            ui.end_list();
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'k' => if todo_curr > 0 {
                todo_curr -= 1;
            },
            'j' => todo_curr = min(todo_curr + 1, todos.len() - 1),
            _ => {}
        }
    }
    getch();
    endwin();
}
