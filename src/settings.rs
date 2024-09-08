use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path;

use super::timer::MyToType;

use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;

use rodio::{source::Source, Decoder, OutputStream};

#[derive(Serialize, Deserialize, Debug)]
struct Timer {
    majitime_min: usize,
    majitime_sec: usize,
    l_min: usize,
    l_sec: usize,
    k: f64,
    w0_min: usize,
    w0_sec: usize,
}

impl Timer {
    fn template() -> Self {
        Self {
            majitime_min: 0,
            majitime_sec: 30,
            l_min: 40,
            l_sec: 0,
            k: 0.0017,
            w0_min: 40,
            w0_sec: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Other {
    pub remind: usize,
    finish_sound: String,
    restart_sound: String,
    remind_sound: String,
}

#[derive(thiserror::Error, Debug)]
pub enum GetPathErr {
    #[error("パスが設定されていません")]
    NoPath,
}

impl Other {
    fn template() -> Self {
        Self {
            remind: 10,
            finish_sound: String::new(),
            restart_sound: String::new(),
            remind_sound: String::new(),
        }
    }
    pub fn set_finish_sound(&mut self, finish_sound_path: &path::Path) {
        self.finish_sound = finish_sound_path.to_string_lossy().into_owned();
        // self.finish_sound_print = finish_sound_path
        //     .file_name()
        //     .unwrap()
        //     .to_string_lossy()
        //     .into_owned();
    }
    pub fn set_restart_sound(&mut self, restart_sound_path: &path::Path) {
        self.restart_sound = restart_sound_path.to_string_lossy().into_owned();
        // self.restart_sound_print = restart_sound_path
        //     .file_name()
        //     .unwrap()
        //     .to_string_lossy()
        //     .into_owned();
    }
    pub fn set_remind_sound(&mut self, remind_sound_path: &path::Path) {
        self.remind_sound = remind_sound_path.to_string_lossy().into_owned();
        // self.remind_sound_print = remind_sound_path
        //     .file_name()
        //     .unwrap()
        //     .to_string_lossy()
        //     .into_owned();
    }
    pub fn get_finish_sound(&self) -> Result<&str, GetPathErr> {
        if &self.finish_sound == "" {
            Err(GetPathErr::NoPath)
        } else {
            Ok(&self.finish_sound)
        }
    }
    pub fn get_restart_sound(&self) -> Result<&str, GetPathErr> {
        if &self.restart_sound == "" {
            Err(GetPathErr::NoPath)
        } else {
            Ok(&self.restart_sound)
        }
    }
    pub fn get_remind_sound(&self) -> Result<&str, GetPathErr> {
        if &self.remind_sound == "" {
            Err(GetPathErr::NoPath)
        } else {
            Ok(&self.remind_sound)
        }
    }
    // pub fn get_finish_sound_print(&self) -> Result<&str, GetPathErr> {
    //     if &self.finish_sound_print == "" {
    //         Err(GetPathErr::NoPath)
    //     } else {
    //         Ok(&self.finish_sound_print)
    //     }
    // }
    // pub fn get_restart_sound_print(&self) -> Result<&str, GetPathErr> {
    //     if &self.restart_sound_print == "" {
    //         Err(GetPathErr::NoPath)
    //     } else {
    //         Ok(&self.restart_sound_print)
    //     }
    // }
    // pub fn get_remind_sound_print(&self) -> Result<&str, GetPathErr> {
    //     if &self.remind_sound_print == "" {
    //         Err(GetPathErr::NoPath)
    //     } else {
    //         Ok(&self.remind_sound_print)
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub timer: Timer,
    pub other: Other,
}

#[derive(thiserror::Error, Debug)]
pub enum SettingsErr {
    #[error("ファイル読み込みに失敗しました")]
    Io(#[from] std::io::Error),
    #[error("serde_jsonの書き込みに失敗しました")]
    SerdeWrite(#[from] serde_json::Error),
    #[error("serde_jsonの読み込みに失敗しました")]
    SerdeRead,
    #[error("{0:?}は存在しません")]
    NoFile(std::ffi::OsString),
    #[error("保存をキャンセルしました")]
    SaveCanceled,
}

impl Settings {
    pub fn template() -> Self {
        Self {
            timer: Timer::template(),
            other: Other::template(),
        }
    }
    pub fn init() -> Result<Self, SettingsErr> {
        let input = dirs::config_dir().unwrap()
            .join("majitimer")
            .join("config.json");
        if input.exists() {
            Ok(Self::import(&input).unwrap())
        } else {
            let template = Self::template();
            template.export(&input).unwrap();
            Ok(template)
        }
    }
    pub fn import(path: &path::Path) -> Result<Self, SettingsErr> {
        let input = std::fs::read_to_string(path).unwrap();
        // 指定したパスのjsonファイルが存在する場合はそこから読み込む。
        if let Ok(deserialized) = serde_json::from_str(&input) {
            return Ok(deserialized);
        } else {
            return Err(SettingsErr::SerdeRead);
        }
    }
    pub fn export(&self, path: &path::Path) -> Result<(), SettingsErr> {
        let parent_dir = path.parent().unwrap();
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
        let mut file = std::fs::File::create(&path).expect(&format!("{:?}", path));
        file.write_all(serde_json::to_string_pretty(&self).unwrap().as_bytes()).unwrap();

        Ok(())
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub fn path_picker(save_file: bool) -> Result<path::PathBuf, SettingsErr> {
    let path = dirs::home_dir().unwrap();
    let builder = rfd::FileDialog::new().set_directory(&path);

    if save_file {
        Ok(builder
            .set_file_name(".json")
            .save_file()
            .ok_or(SettingsErr::SaveCanceled)?)
    } else {
        Ok(builder.pick_file().ok_or(SettingsErr::SaveCanceled)?)
    }
}

#[derive(Debug, PartialEq)]
pub enum TimerMode {
    Init,
    MajiTime,
    Endurance,
    Rest,
    UrgedToReMajiTime,
}

pub struct RunData {
    rodio: (rodio::OutputStream, rodio::OutputStreamHandle),
    rodio_sink: Option<rodio::Sink>,
    paused: bool,
    mode_transition: bool,
    mode: TimerMode,
    majitime: std::time::Duration,
    remind: std::time::Duration,
    l: usize,
    k: f64,
    w0: usize,
    up: super::timer::Timer,
    down: super::timer::CountDownTimer,
}

#[derive(thiserror::Error, Debug)]
pub enum RunDataErr {
    #[error("ファイル読み込みに失敗しました")]
    Io(#[from] std::io::Error),
    #[error("タイマーポーズ中です")]
    Paused,
    #[error("音楽プレイヤーの再生に失敗しました")]
    Rodio(#[from] rodio::PlayError),
}

impl RunData {
    pub fn new() -> Self {
        Self {
            rodio: rodio::OutputStream::try_default().unwrap(),
            rodio_sink: None,
            paused: true,
            mode_transition: false,
            mode: TimerMode::Init,
            majitime: std::time::Duration::ZERO,
            remind: std::time::Duration::ZERO,
            l: 0,
            k: 0.0,
            w0: 0,
            up: super::timer::Timer::new(),
            down: super::timer::CountDownTimer::new(),
        }
    }
    pub fn init(&mut self, settings: &Settings) {
        self.majitime = std::time::Duration::from_secs(
            (settings.timer.majitime_min * 60 + settings.timer.majitime_sec) as u64,
        );
        self.remind = std::time::Duration::from_secs(settings.other.remind as u64);
        self.l = settings.timer.l_min * 60 + settings.timer.l_sec;
        self.k = settings.timer.k;
        self.w0 = settings.timer.w0_min * 60 + settings.timer.w0_sec;

        self.mode = TimerMode::MajiTime;
        self.paused = false;
        self.down.init(self.majitime);
    }
    pub fn update(&mut self, settings: &Settings) {
        self.majitime = std::time::Duration::from_secs(
            (settings.timer.majitime_min * 60 + settings.timer.majitime_sec) as u64,
        );
        self.remind = std::time::Duration::from_secs(settings.other.remind as u64);
        self.l = settings.timer.l_min * 60 + settings.timer.l_sec;
        self.k = settings.timer.k;
        self.w0 = settings.timer.w0_min * 60 + settings.timer.w0_sec;
    }
    pub fn pause_or_resume(&mut self) {
        if self.paused {
            match self.mode {
                TimerMode::MajiTime | TimerMode::Rest | TimerMode::UrgedToReMajiTime => {
                    self.down.resume()
                }
                TimerMode::Endurance => self.up.resume(),
                _ => unreachable!(),
            }
        } else {
            match self.mode {
                TimerMode::MajiTime | TimerMode::Rest | TimerMode::UrgedToReMajiTime => {
                    self.down.pause()
                }
                TimerMode::Endurance => self.up.pause(),
                _ => unreachable!(),
            }
        }
        self.paused = !self.paused;
    }
    // r = L / (1 + e^(-k * (w - w_0)))
    fn calc_rest_time(&self) -> std::time::Duration {
        let w = self.up.get_time().as_secs() as f64;
        let r = self.l as f64 / (1.0 + std::f64::consts::E.powf(-self.k * (w - self.w0 as f64)));
        std::time::Duration::from_secs_f64(r)
    }
    fn play_sound(&mut self, path: Result<&str, GetPathErr>) -> Result<(), RunDataErr> {
        // assert_eq!(path.is_ok(), false);
        if let Ok(path) = path {
            // assert_eq!(path, "assets/finish.mp3");
            // let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            // let sink = rodio::Sink::try_new(&handle).unwrap();
            //
            // let file = std::fs::File::open(path)?;
            // sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).unwrap());
            // sink.set_volume(0.05);
            // sink.sleep_until_end();
            // let file = std::io::BufReader::new(std::fs::File::open(path)?);
            // let source = rodio::Decoder::new(file).unwrap();
            // handle.play_raw(source.convert_samples()).unwrap();
            // sink.detach();
            // let audacity = handle
            //     .play_once(std::io::BufReader::new(file))?;
            // audacity.set_volume(0.05);

            let file = std::fs::File::open(path)?;
            self.rodio_sink = Some(self.rodio.1.play_once(std::io::BufReader::new(file))?);
            self.rodio_sink.as_ref().unwrap().set_volume(0.1);
        }
        Ok(())
    }
    pub fn mode_transition_start(&mut self) {
        self.mode_transition = true;
    }
    pub fn paused(&self) -> bool {
        self.paused
    }
    pub fn mode(&self) -> &TimerMode {
        &self.mode
    }
    pub fn state_process(&mut self, settings: &Settings) -> Result<(), RunDataErr> {
        if self.paused {
            return Ok(());
        }

        match self.mode {
            TimerMode::Init => {
                unreachable!();
            }
            TimerMode::MajiTime => {
                if self.down.get_time() == std::time::Duration::ZERO {
                    self.mode_transition = false;

                    self.mode = TimerMode::Endurance;
                    self.down = super::timer::CountDownTimer::new();

                    self.up.init();

                    self.play_sound(settings.other.get_finish_sound())?;
                }
            }
            TimerMode::Endurance => {
                if self.mode_transition {
                    self.mode_transition = false;

                    self.mode = TimerMode::Rest;
                    self.up = super::timer::Timer::new();

                    self.down.init(self.calc_rest_time());
                    // self.down.init(std::time::Duration::from_secs(30));
                }
            }
            TimerMode::Rest => {
                if self.down.get_time() == std::time::Duration::ZERO {
                    self.mode_transition = false;

                    self.mode = TimerMode::UrgedToReMajiTime;

                    self.down.init(self.remind);

                    self.play_sound(settings.other.get_restart_sound())?;
                }
            }
            TimerMode::UrgedToReMajiTime => {
                if self.mode_transition {
                    self.mode_transition = false;

                    self.mode = TimerMode::MajiTime;
                    self.up = super::timer::Timer::new();

                    self.down.init(self.majitime);

                    return Ok(());
                }
                // リマインドの時間が来たら、タイマーリセット後remind_soundを再生
                if self.down.get_time() == std::time::Duration::ZERO {
                    self.down.init(self.remind);
                    self.play_sound(settings.other.get_remind_sound())?;
                }
            }
        }

        Ok(())
    }
    pub fn render_time(&self) -> String {
        match self.mode {
            TimerMode::Init => "UNREACHABLE".to_string(),
            TimerMode::MajiTime | TimerMode::Rest | TimerMode::UrgedToReMajiTime => {
                self.down.get_time().to_time_string()
            }
            TimerMode::Endurance => self.up.get_time().to_time_string(),
        }
    }
    /// ratatuiでレンダリングする文字列のリストを返す
    /// * .0 => タイマーの説明用文字列
    /// * 0 : 現在のTimerModeの名前 or アプリケーションの名前
    /// * 1 : タイマーの現在の時間 or アプリケーションの説明
    /// * 2 : 現在のTimerModeの説明
    ///
    /// * .1 => キーヒント表示用文字列
    pub fn render(&self) -> (Vec<Line>, Vec<Span>) {
        let except_init: Vec<ratatui::text::Span> = vec![
            " ポーズ/再開 ".into(),
            "<Space> ".blue().bold(),
            " リセット ".into(),
            "<R> ".blue().bold(),
            " 設定のインポート ".into(),
            "<I> ".blue().bold(),
            " 設定のエクスポート ".into(),
            "<E> ".blue().bold(),
            " 終了 ".into(),
            "<Q> ".blue().bold(),
        ];
        match self.mode {
            TimerMode::Init => (
                vec![
                    Line::from("本気タイマー v0.1.0".white().bold()),
                    Line::from("頑張った分だけ休憩が非線形におおよそ多くなるタイマーです"),
                    Line::from("ここに現在のタイマーのモードの説明が表示されます".bold()),
                ],
                vec![
                    " スタート ".into(),
                    "<Space> ".blue().bold(),
                    " 設定のインポート ".into(),
                    "<I> ".blue().bold(),
                    " 設定のエクスポート ".into(),
                    "<E> ".blue().bold(),
                    " 終了 ".into(),
                    "<Q> ".blue().bold(),
                ],
            ),
            TimerMode::MajiTime => (
                vec![
                    Line::from("本気モード\n".red().bold()),
                    Line::from(self.down.get_time().to_time_string()),
                    Line::from("短時間だけ本気で作業をする時間です。休憩厳禁！だけど超短い！"),
                ],
                except_init,
            ),
            TimerMode::Endurance => (
                vec![
                    Line::from(" 耐久モード\n".green().bold()),
                    Line::from(self.up.get_time().to_time_string()),
                    Line::from(vec![
                        "もうやりたくないと思うまで作業をする時間です。".into(),
                        " <M> ".blue().bold(),
                        "で休憩に入ることができますが、長く作業すればその分長く休憩できます。\n"
                            .into(),
                    ]),
                ],
                except_init,
            ),
            TimerMode::Rest => (
                vec![
                    Line::from(" 休憩モード \n".cyan().bold()),
                    Line::from(self.down.get_time().to_time_string()),
                    Line::from(
                        "耐久モードで作業した時間に応じた休憩を取る時間です。十分に休みましょう。",
                    ),
                ],
                except_init,
            ),
            TimerMode::UrgedToReMajiTime => (
                vec![
                    Line::from(" 本気モードを手動で開始してください \n".yellow().bold()),
                    Line::from(self.down.get_time().to_time_string()),
                    Line::from(vec![
                        "休憩モードでの休憩時間が終わりました。".into(),
                        " <M> ".blue().bold(),
                        "で再び本気で作業をしましょう。\n".into(),
                    ]),
                ],
                except_init,
            ),
        }
    }
}
