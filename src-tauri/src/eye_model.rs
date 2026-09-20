use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub language: String,
    pub main_theme: String,
    pub tray_theme: String,
    pub tray_show_chart: bool,
    pub overlay_opacity: u32,
    pub overlay_blur: u32,
    pub work_minutes: u32,
    pub break_minutes: u32,
    pub break_seconds: u32,
    pub repeat_minutes: u32,
    pub pre_notify: bool,
    pub sound: bool,
    pub movie_mode: bool,
    pub autostart: bool,
    pub tray_icon: bool,
    pub tray_time: bool,
    pub dock_icon: bool,
    pub reminders: bool,
    pub reminder_style: String,
    pub message: String,
    pub allow_skip: bool,
    pub background: String,
    pub background_image: String,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            language: crate::i18n::system_language(),
            main_theme: "dark".into(),
            tray_theme: "dark".into(),
            tray_show_chart: true,
            overlay_opacity: 82,
            overlay_blur: 16,
            work_minutes: 25,
            break_minutes: 5,
            break_seconds: 0,
            repeat_minutes: 1,
            pre_notify: true,
            sound: true,
            movie_mode: false,
            autostart: false,
            tray_icon: true,
            tray_time: true,
            dock_icon: true,
            reminders: true,
            reminder_style: "fullscreen".into(),
            message: String::new(),
            allow_skip: true,
            background: "system".into(),
            background_image: String::new(),
        }
    }
}
impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        if !crate::i18n::LANGUAGES.contains(&self.language.as_str()) {
            return Err("Unsupported language".into());
        }
        if !["system", "light", "dark"].contains(&self.main_theme.as_str())
            || !["system", "light", "dark"].contains(&self.tray_theme.as_str())
            || !(50..=100).contains(&self.overlay_opacity)
            || self.overlay_blur > 40
        {
            return Err(crate::i18n::text(&self.language, "主题设置无效：遮罩浓度须为 50–100%，模糊须为 0–40"));
        }
        if !(1..=240).contains(&self.work_minutes)
            || self.break_minutes > 60
            || self.break_seconds > 59
            || self.break_duration() == 0
            || !(1..=60).contains(&self.repeat_minutes)
        {
            return Err(
                crate::i18n::text(&self.language, "工作时长须为 1–240 分钟，休息时长须为 1 秒–60 分 59 秒，提醒间隔须为 1–60 分钟"),
            );
        }
        if !["fullscreen", "window"].contains(&self.reminder_style.as_str())
            || !["system", "light", "dark", "custom"].contains(&self.background.as_str())
            || self.message.chars().count() > 120
            || self.background_image.len() > 4_200_000
        {
            return Err(crate::i18n::text(&self.language, "提醒设置无效，图片须小于 3 MB，提醒文字不能超过 120 字"));
        }
        if !self.background_image.is_empty()
            && ![
                "data:image/png;base64,",
                "data:image/jpeg;base64,",
                "data:image/webp;base64,",
            ]
            .iter()
            .any(|p| self.background_image.starts_with(p))
        {
            return Err(crate::i18n::text(&self.language, "请选择 PNG、JPEG 或 WebP 图片"));
        }
        if !self.tray_icon && !self.dock_icon {
            return Err(crate::i18n::text(&self.language, "请至少保留状态栏或程序坞入口"));
        }
        Ok(())
    }
    pub fn break_duration(&self) -> u32 {
        self.break_minutes * 60 + self.break_seconds
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Day {
    pub seconds: u64,
    pub peak_seconds: u64,
    pub breaks: u32,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Monitor {
    pub settings: Settings,
    pub days: BTreeMap<String, Day>,
    pub date: String,
    pub samples: Vec<(u32, u32)>,
    pub work_seconds: f64,
    pub paused_until: i64,
    pub break_until: i64,
    pub next_reminder: i64,
    #[serde(skip)]
    pub last_tick: i64,
    #[serde(skip)]
    pub storage_error: Option<String>,
}
impl Default for Monitor {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            days: BTreeMap::new(),
            date: String::new(),
            samples: vec![],
            work_seconds: 0.0,
            paused_until: 0,
            break_until: 0,
            next_reminder: 0,
            last_tick: 0,
            storage_error: None,
        }
    }
}
#[derive(Default, PartialEq, Debug)]
pub enum Effect {
    #[default]
    None,
    Warn,
    StartBreak,
    EndBreak,
}
impl Monitor {
    pub fn fatigue(&self) -> u32 {
        (self.work_seconds / (self.settings.work_minutes.max(1) * 60) as f64 * 100.0)
            .clamp(0.0, 100.0)
            .round() as u32
    }
    pub fn start_break(&mut self, now: i64) {
        self.break_until = now + self.settings.break_duration() as i64;
    }
    pub fn tick(&mut self, now: i64, date: &str, minute: u32, idle: bool) -> Effect {
        if self.date != date {
            self.date = date.into();
            self.samples.clear();
        }
        self.days.entry(date.into()).or_default();
        while self.days.len() > 3660 {
            if let Some(first) = self.days.keys().next().cloned() {
                self.days.remove(&first);
            }
        }
        let elapsed = if self.last_tick == 0 {
            0
        } else {
            (now - self.last_tick).clamp(0, 2) as u64
        };
        if self.last_tick > 0 && now - self.last_tick >= self.settings.break_duration() as i64 {
            self.work_seconds = 0.0;
        }
        self.last_tick = now;
        let mut effect = Effect::None;
        if self.break_until > 0 {
            if now >= self.break_until {
                self.break_until = 0;
                self.work_seconds = 0.0;
                self.next_reminder = 0;
                self.days.get_mut(date).unwrap().breaks += 1;
                effect = Effect::EndBreak;
            }
        } else if idle && !self.settings.movie_mode {
            self.work_seconds = (self.work_seconds
                - elapsed as f64 * self.settings.work_minutes as f64 * 60.0
                    / self.settings.break_duration().max(1) as f64)
                .max(0.0);
        } else {
            self.work_seconds += elapsed as f64;
            self.days.get_mut(date).unwrap().seconds += elapsed;
            if self.fatigue() >= 100 {
                self.days.get_mut(date).unwrap().peak_seconds += elapsed;
            }
            if self.settings.reminders && now >= self.paused_until {
                let remaining = self.settings.work_minutes as f64 * 60.0 - self.work_seconds;
                if self.settings.pre_notify
                    && remaining > 0.0
                    && remaining <= 10.0
                    && remaining + elapsed as f64 > 10.0
                {
                    effect = Effect::Warn;
                }
                if remaining <= 0.0 && now >= self.next_reminder {
                    self.start_break(now);
                    effect = Effect::StartBreak;
                }
            }
        }
        let point = (minute, self.fatigue());
        if self.samples.last().map(|p| p.0) == Some(minute) {
            *self.samples.last_mut().unwrap() = point;
        } else {
            self.samples.push(point);
        }
        effect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_settings_keep_appearance_and_themes_roundtrip_independently() {
        let mut s: Settings = serde_json::from_str(r#"{"background":"custom","workMinutes":30}"#).unwrap();
        assert_eq!(s.main_theme, "dark");
        assert_eq!(s.tray_theme, "dark");
        assert_eq!(s.background, "custom");
        assert_eq!(s.overlay_opacity, 82);
        assert_eq!(s.overlay_blur, 16);
        assert!(s.tray_show_chart);
        s.language = "ja".into();
        s.main_theme = "light".into();
        s.tray_theme = "system".into();
        s.tray_show_chart = false;
        let restored: Settings = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert_eq!(restored.language, "ja");
        assert_eq!(restored.main_theme, "light");
        assert_eq!(restored.tray_theme, "system");
        assert_eq!(restored.background, "custom");
        assert!(!restored.tray_show_chart);
        assert!(restored.validate().is_ok());
    }
    #[test]
    fn appearance_rejects_unknown_themes_and_out_of_range_effects() {
        let mut s = Settings::default();
        s.main_theme = "unknown".into();
        assert!(s.validate().is_err());
        s.main_theme = "system".into();
        s.tray_theme = "unknown".into();
        assert!(s.validate().is_err());
        s.tray_theme = "light".into();
        s.overlay_opacity = 49;
        assert!(s.validate().is_err());
        s.overlay_opacity = 100;
        s.overlay_blur = 41;
        assert!(s.validate().is_err());
        s.overlay_blur = 0;
        assert!(s.validate().is_ok());
    }
    #[test]
    fn work_reaches_break_and_completes_once() {
        let mut m = Monitor::default();
        m.settings.work_minutes = 1;
        m.settings.break_minutes = 0;
        m.settings.break_seconds = 2;
        for n in 1..=60 {
            m.tick(n, "2026-09-20", 0, false);
        }
        assert_eq!(m.tick(61, "2026-09-20", 1, false), Effect::StartBreak);
        assert_eq!(m.days["2026-09-20"].seconds, 60);
        assert_eq!(m.tick(63, "2026-09-20", 1, false), Effect::EndBreak);
        assert_eq!(m.fatigue(), 0);
        m.tick(64, "2026-09-20", 1, false);
        assert_eq!(m.days["2026-09-20"].breaks, 1);
    }
    #[test]
    fn pause_suppresses_reminders_but_keeps_usage() {
        let mut m = Monitor::default();
        m.settings.work_minutes = 1;
        m.work_seconds = 60.0;
        m.last_tick = 10;
        m.paused_until = 20;
        assert_eq!(m.tick(11, "2026-09-20", 0, false), Effect::None);
        assert_eq!(m.days["2026-09-20"].seconds, 1);
        assert_eq!(m.tick(20, "2026-09-20", 0, false), Effect::StartBreak);
    }
    #[test]
    fn idle_recovers_and_sleep_is_not_counted() {
        let mut m = Monitor::default();
        m.work_seconds = 100.0;
        m.last_tick = 1;
        m.tick(3600, "2026-09-20", 60, true);
        assert_eq!(m.days["2026-09-20"].seconds, 0);
        assert!(m.work_seconds < 100.0);
        m.tick(7200, "2026-09-20", 120, false);
        assert_eq!(m.days["2026-09-20"].seconds, 2);
    }
    #[test]
    fn midnight_resets_chart_not_history() {
        let mut m = Monitor::default();
        m.tick(1, "2026-09-20", 1439, false);
        m.tick(2, "2026-09-21", 0, false);
        assert_eq!(m.samples.len(), 1);
        assert_eq!(m.days.len(), 2);
        assert_eq!(m.days["2026-09-21"].seconds, 1);
    }
    #[test]
    fn settings_reject_zero_break_and_inaccessible_app() {
        let mut s = Settings::default();
        s.break_minutes = 0;
        assert!(s.validate().is_err());
        s.break_seconds = 20;
        assert!(s.validate().is_ok());
        s.tray_icon = false;
        s.dock_icon = false;
        assert!(s.validate().is_err());
    }
    #[test]
    fn persistence_roundtrip_preserves_history() {
        let mut m = Monitor::default();
        m.tick(1, "2026-09-20", 10, false);
        m.tick(2, "2026-09-20", 10, false);
        let restored: Monitor = serde_json::from_str(&serde_json::to_string(&m).unwrap()).unwrap();
        assert_eq!(restored.days["2026-09-20"].seconds, 1);
        assert_eq!(restored.last_tick, 0);
    }
}
