use crossterm::event::KeyCode;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
    Rng,
};
use tui::widgets::ListState;

const TASKS: [&str; 4] = ["Item1", "Item2", "Item3", "Item4"];

const LOGS: [(&str, &str); 48] = [
    ("TPE launch system is ready and stable", "INFO"),
    ("LAX launch system is ready and stable", "INFO"),
    ("JFK launch system is ready and stable", "INFO"),
    (
        "Unknown connection detected from 140.122.64.120 (Taiwan)",
        "WARNING",
    ),
    ("SFO launch system is ready and stable", "INFO"),
    ("ORD launch system is ready and stable", "INFO"),
    ("DFW launch system is ready and stable", "INFO"),
    (
        "Lost connection from LAX launch system due to internal errors",
        "ERROR",
    ),
    ("DEFCON alert has been raised to level 3", "CRITICAL"),
    ("MIA launch system is ready and stable", "INFO"),
    ("SEA launch system is ready and stable", "INFO"),
    (
        "Connection established with ISS (International Space Station)",
        "INFO",
    ),
    ("Received telemetry data from TPE launch system", "INFO"),
    ("Error reading sensor data from JFK launch system", "ERROR"),
    ("Power outage detected at LAX launch system", "WARNING"),
    (
        "Authentication failure from 192.168.0.1 (internal network)",
        "WARNING",
    ),
    ("Launch countdown initiated for ORD launch system", "INFO"),
    (
        "Critical system update installed on DFW launch system",
        "INFO",
    ),
    (
        "Network congestion observed on SFO launch system",
        "WARNING",
    ),
    (
        "Data corruption detected in MIA launch system logs",
        "ERROR",
    ),
    ("Launch sequence aborted for SEA launch system", "CRITICAL"),
    ("System overload on TPE launch system", "WARNING"),
    ("Launch system reconfigured successfully at JFK", "INFO"),
    (
        "Unauthorized access attempt detected on ORD launch system",
        "WARNING",
    ),
    (
        "Emergency shutdown triggered on LAX launch system",
        "CRITICAL",
    ),
    ("Connection established with weather satellite", "INFO"),
    ("Error writing log file on DFW launch system", "ERROR"),
    (
        "Unexpected response received from SFO launch system",
        "WARNING",
    ),
    ("Insufficient fuel detected in MIA launch system", "ERROR"),
    ("Launch system rebooted successfully at SEA", "INFO"),
    (
        "Unauthorized login attempt from 192.168.0.10 (internal network)",
        "WARNING",
    ),
    (
        "Critical hardware failure reported by LAX launch system",
        "CRITICAL",
    ),
    ("Security breach detected on SFO launch system", "CRITICAL"),
    ("Power supply failure on DFW launch system", "ERROR"),
    (
        "Communication error with satellite at MIA launch system",
        "ERROR",
    ),
    (
        "Launch aborted due to inclement weather at SEA launch system",
        "CRITICAL",
    ),
    (
        "Unauthorized access attempt from external IP 203.120.45.78",
        "WARNING",
    ),
    (
        "Critical software bug discovered on TPE launch system",
        "CRITICAL",
    ),
    ("Sensor malfunction detected on JFK launch system", "ERROR"),
    ("Network outage affecting ORD launch system", "WARNING"),
    ("Launch system initialization completed at LAX", "INFO"),
    (
        "Unauthorized login detected on DFW launch system",
        "WARNING",
    ),
    (
        "Satellite signal loss experienced on SFO launch system",
        "WARNING",
    ),
    ("Fuel leak detected in MIA launch system", "ERROR"),
    ("Launch system update deployed successfully at SEA", "INFO"),
    (
        "Unauthorized network scan detected on TPE launch system",
        "WARNING",
    ),
    (
        "Critical failure in propulsion system on JFK launch system",
        "CRITICAL",
    ),
    (
        "Error in communication protocol with ISS on ORD launch system",
        "ERROR",
    ),
];

const EVENTS: [(&str, u64); 48] = [
    ("TPE", 9),
    ("LAX", 5),
    ("JFK", 8),
    ("SFO", 3),
    ("ORD", 4),
    ("DFW", 7),
    ("MIA", 2),
    ("SEA", 1),
    ("CLT", 10),
    ("LAS", 7),
    ("PHX", 6),
    ("MCO", 3),
    ("IAH", 5),
    ("EWR", 8),
    ("ATL", 9),
    ("LGA", 4),
    ("DEN", 2),
    ("BOS", 1),
    ("SFO", 3),
    ("PHX", 6),
    ("MCO", 3),
    ("IAH", 5),
    ("EWR", 8),
    ("ATL", 5),
    ("LGA", 4),
    ("DEN", 2),
    ("BOS", 1),
    ("SFO", 3),
    ("PHX", 6),
    ("MCO", 3),
    ("IAH", 5),
    ("EWR", 8),
    ("ATL", 10),
    ("LGA", 4),
    ("DEN", 2),
    ("BOS", 1),
    ("SFO", 3),
    ("PHX", 6),
    ("MCO", 3),
    ("IAH", 5),
    ("EWR", 8),
    ("ATL", 7),
    ("LGA", 4),
    ("DEN", 2),
    ("BOS", 1),
    ("SFO", 3),
    ("PHX", 6),
    ("MCO", 3),
];

const LAUNCHERS: [Launcher; 34] = [
    Launcher {
        name: "Asia-1",
        location: "TPE",
        coords: (25.0094715, 121.5370432),
        status: "Up",
    },
    Launcher {
        name: "USA-1",
        location: "LAX",
        coords: (34.052235, -118.243683),
        status: "Up",
    },
    Launcher {
        name: "USA-2",
        location: "JFK",
        coords: (40.6413111, -73.7781391),
        status: "Down",
    },
    Launcher {
        name: "USA-3",
        location: "SFO",
        coords: (37.7749, -122.4194),
        status: "Up",
    },
    Launcher {
        name: "USA-4",
        location: "ORD",
        coords: (41.9742, -87.9073),
        status: "Down",
    },
    Launcher {
        name: "USA-5",
        location: "DFW",
        coords: (32.8998, -97.0403),
        status: "Up",
    },
    Launcher {
        name: "USA-6",
        location: "MIA",
        coords: (25.7617, -80.1918),
        status: "Down",
    },
    Launcher {
        name: "USA-7",
        location: "SEA",
        coords: (47.6062, -122.3321),
        status: "Up",
    },
    Launcher {
        name: "USA-8",
        location: "CLT",
        coords: (35.2271, -80.8431),
        status: "Down",
    },
    Launcher {
        name: "USA-9",
        location: "LAS",
        coords: (36.1699, -115.1398),
        status: "Up",
    },
    Launcher {
        name: "Europe-1",
        location: "LHR",
        coords: (51.5074, -0.1278),
        status: "Up",
    },
    Launcher {
        name: "Europe-2",
        location: "CDG",
        coords: (48.8566, 2.3522),
        status: "Down",
    },
    Launcher {
        name: "Asia-2",
        location: "HND",
        coords: (35.6895, 139.6917),
        status: "Up",
    },
    Launcher {
        name: "Asia-3",
        location: "ICN",
        coords: (37.5665, 126.9780),
        status: "Down",
    },
    Launcher {
        name: "Africa-1",
        location: "JNB",
        coords: (-26.2041, 28.0473),
        status: "Up",
    },
    Launcher {
        name: "Africa-2",
        location: "CAI",
        coords: (30.0444, 31.2357),
        status: "Down",
    },
    Launcher {
        name: "Australia-1",
        location: "SYD",
        coords: (-33.8688, 151.2093),
        status: "Up",
    },
    Launcher {
        name: "SouthAmerica-1",
        location: "GRU",
        coords: (-23.5505, -46.6333),
        status: "Up",
    },
    Launcher {
        name: "Europe-3",
        location: "FRA",
        coords: (50.1109, 8.6821),
        status: "Up",
    },
    Launcher {
        name: "Europe-4",
        location: "MAD",
        coords: (40.4168, -3.7038),
        status: "Up",
    },
    Launcher {
        name: "Asia-4",
        location: "PVG",
        coords: (31.2304, 121.4737),
        status: "Up",
    },
    Launcher {
        name: "Asia-5",
        location: "BOM",
        coords: (19.0760, 72.8777),
        status: "Down",
    },
    Launcher {
        name: "Africa-3",
        location: "LOS",
        coords: (6.5244, 3.3792),
        status: "Up",
    },
    Launcher {
        name: "Africa-4",
        location: "NBO",
        coords: (-1.2864, 36.8172),
        status: "Down",
    },
    Launcher {
        name: "Australia-2",
        location: "MEL",
        coords: (-37.8136, 144.9631),
        status: "Down",
    },
    Launcher {
        name: "SouthAmerica-2",
        location: "EZE",
        coords: (-34.6037, -58.3816),
        status: "Down",
    },
    Launcher {
        name: "Europe-5",
        location: "AMS",
        coords: (52.3676, 4.9041),
        status: "Up",
    },
    Launcher {
        name: "Europe-6",
        location: "FCO",
        coords: (41.9028, 12.4964),
        status: "Down",
    },
    Launcher {
        name: "Asia-6",
        location: "SIN",
        coords: (1.3521, 103.8198),
        status: "Up",
    },
    Launcher {
        name: "Asia-7",
        location: "BKK",
        coords: (13.7563, 100.5018),
        status: "Down",
    },
    Launcher {
        name: "Africa-5",
        location: "CPT",
        coords: (-33.9249, 18.4241),
        status: "Up",
    },
    Launcher {
        name: "Africa-6",
        location: "ALG",
        coords: (36.7372, 3.0865),
        status: "Up",
    },
    Launcher {
        name: "Australia-3",
        location: "BNE",
        coords: (-27.4698, 153.0251),
        status: "Up",
    },
    Launcher {
        name: "SouthAmerica-3",
        location: "LIM",
        coords: (-12.0464, -77.0428),
        status: "Down",
    },
];

#[derive(Clone)]
pub struct RandomSignal {
    distribution: Uniform<u64>,
    rng: ThreadRng,
}

impl RandomSignal {
    pub fn new(lower: u64, upper: u64) -> RandomSignal {
        RandomSignal {
            distribution: Uniform::new(lower, upper),
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

#[derive(Clone)]
pub struct CorruptedSinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl CorruptedSinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> CorruptedSinSignal {
        CorruptedSinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for CorruptedSinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (
            self.x,
            ((self.x * 1.0 / self.period).sin() + rand::thread_rng().gen_range(-0.1..0.1))
                * self.scale,
        );
        self.x += self.interval;
        Some(point)
    }
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct Signals {
    pub sin1: Signal<CorruptedSinSignal>,
    pub sin2: Signal<CorruptedSinSignal>,
    pub window: [f64; 2],
}

impl Signals {
    fn on_tick(&mut self) {
        self.sin1.on_tick();
        self.sin2.on_tick();
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

#[derive(Clone)]
pub struct Launcher<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
    pub status: &'a str,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub progress: f64,
    pub sparkline: Signal<RandomSignal>,
    pub tasks: StatefulList<&'a str>,
    pub logs: StatefulList<(&'a str, &'a str)>,
    pub signals: Signals,
    pub packets: Vec<(&'a str, u64)>,
    pub launchers: Vec<Launcher<'a>>,
    pub power: f64,
    pub code: String,
    pub correct_code: String,
    pub typing: bool,
    pub missile_launched: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, correct_code: String) -> App<'a> {
        let mut rand_signal = RandomSignal::new(0, 100);
        let sparkline_points = rand_signal.by_ref().take(300).collect();
        let mut sin_signal = CorruptedSinSignal::new(0.2, 3.0, 16.0);
        let sin1_points = sin_signal.by_ref().take(100).collect();
        let mut sin_signal2 = CorruptedSinSignal::new(0.1, 2.0, 8.0);
        let sin2_points = sin_signal2.by_ref().take(200).collect();
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["System Monitor", "Launch Missile"]),
            progress: 0.0,
            sparkline: Signal {
                source: rand_signal,
                points: sparkline_points,
                tick_rate: 1,
            },
            tasks: StatefulList::with_items(TASKS.to_vec()),
            logs: StatefulList::with_items(LOGS.to_vec()),
            signals: Signals {
                sin1: Signal {
                    source: sin_signal,
                    points: sin1_points,
                    tick_rate: 5,
                },
                sin2: Signal {
                    source: sin_signal2,
                    points: sin2_points,
                    tick_rate: 10,
                },
                window: [0.0, 20.0],
            },
            packets: EVENTS.to_vec(),
            launchers: LAUNCHERS.to_vec(),
            power: 50.0,
            code: String::new(),
            typing: false,
            missile_launched: false,
            correct_code,
        }
    }

    pub fn on_up(&mut self) {
        self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        self.tasks.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: KeyCode) {
        if self.typing {
            match c {
                KeyCode::Enter => {
                    if self.code == self.correct_code {
                        self.missile_launched = true;
                        self.typing = false;
                    }
                }
                KeyCode::Backspace | KeyCode::Delete => {
                    self.code.pop();
                }
                KeyCode::Esc => {
                    self.code.clear();
                    self.typing = false;
                }
                KeyCode::Char(c) => {
                    self.code.push(c);
                }
                _ => {}
            }
        } else {
            match c {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                KeyCode::Char('t') => {
                    self.typing = true;
                }
                _ => {}
            }
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }

        self.sparkline.on_tick();
        self.signals.on_tick();

        if (self.progress * 1000.0) as i64 % 5 == 0 {
            let log = self.logs.items.pop().unwrap();
            self.logs.items.insert(0, log);
        }

        if (self.progress * 1000.0) as i64 % 3 == 0 {
            let event = self.packets.pop().unwrap();
            self.packets.insert(0, event);
        }

        if (self.progress * 1000.0) as i64 % 10 == 0 {
            self.power = (self.power + rand::random::<f64>() * 50.0 - 25.0)
                .max(0.0)
                .min(100.0);
        }
    }
}
