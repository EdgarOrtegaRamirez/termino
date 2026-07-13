use termino::storage::{get_session_count, get_sessions, save_session, SessionData, SessionLap};

fn setup() {
    // Use a temp directory for storage in tests
    let tmp = std::env::temp_dir().join("termino_test");
    let _ = std::fs::remove_dir_all(&tmp);
    unsafe { std::env::set_var("TERMINO_HOME", tmp.to_str().unwrap()); }
}

#[test]
fn test_save_session_creates_file() {
    setup();
    let session = SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 30.5,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    save_session(&session).unwrap();
    let sessions = get_sessions(None, None).unwrap();
    assert!(!sessions.is_empty());
}

#[test]
fn test_save_session_returns_valid_data() {
    setup();
    let session = SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 30.5,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    let saved = save_session(&session).unwrap();
    assert_eq!(saved.session_type, "stopwatch");
    assert!((saved.duration_seconds - 30.5).abs() < 0.001);
    assert_eq!(saved.status, "completed");
}

#[test]
fn test_save_session_with_laps() {
    setup();
    let laps = vec![
        SessionLap { lap: 1, split: 10.0, total: 10.0 },
        SessionLap { lap: 2, split: 15.5, total: 25.5 },
    ];
    let session = SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 25.5,
        status: "completed".to_string(),
        laps: Some(laps),
        cycles: None,
    };
    let saved = save_session(&session).unwrap();
    assert_eq!(saved.laps.as_ref().unwrap().len(), 2);
    assert_eq!(saved.laps.as_ref().unwrap()[0].lap, 1);
}

#[test]
fn test_get_sessions_empty() {
    setup();
    let sessions = get_sessions(None, None).unwrap();
    assert!(sessions.is_empty());
}

#[test]
fn test_get_sessions_returns_all() {
    setup();
    let s1 = SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 30.0,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    let s2 = SessionData {
        session_type: "countdown".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 60.0,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    save_session(&s1).unwrap();
    save_session(&s2).unwrap();
    let sessions = get_sessions(None, None).unwrap();
    assert_eq!(sessions.len(), 2);
}

#[test]
fn test_get_sessions_filter_by_type() {
    setup();
    let s1 = SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 30.0,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    let s2 = SessionData {
        session_type: "countdown".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 60.0,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    save_session(&s1).unwrap();
    save_session(&s2).unwrap();
    let stopwatch_sessions = get_sessions(Some("stopwatch"), None).unwrap();
    assert_eq!(stopwatch_sessions.len(), 1);
}

#[test]
fn test_get_sessions_limit() {
    setup();
    for i in 0..5 {
        let s = SessionData {
            session_type: "stopwatch".to_string(),
            started: chrono::Utc::now().to_rfc3339(),
            ended: chrono::Utc::now().to_rfc3339(),
            duration_seconds: (i * 10) as f64,
            status: "completed".to_string(),
            laps: None,
            cycles: None,
        };
        save_session(&s).unwrap();
    }
    let sessions = get_sessions(None, Some(3)).unwrap();
    assert_eq!(sessions.len(), 3);
}

#[test]
fn test_get_sessions_most_recent_first() {
    setup();
    let s1 = SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 10.0,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    std::thread::sleep(std::time::Duration::from_millis(10));
    let s2 = SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 20.0,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    save_session(&s1).unwrap();
    save_session(&s2).unwrap();
    let sessions = get_sessions(None, None).unwrap();
    assert!((sessions[0].duration_seconds - 20.0).abs() < 0.001);
    assert!((sessions[1].duration_seconds - 10.0).abs() < 0.001);
}

#[test]
fn test_get_session_count() {
    setup();
    assert_eq!(get_session_count(), 0);
    let s = SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: 30.0,
        status: "completed".to_string(),
        laps: None,
        cycles: None,
    };
    save_session(&s).unwrap();
    assert_eq!(get_session_count(), 1);
}