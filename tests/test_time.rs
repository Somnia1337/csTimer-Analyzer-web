use cstimer_analyzer_web::time::*;

#[test]
fn test_to_readable_string() {
    let pairs = [
        (0, "0.000"),
        (1, "0.001"),
        (12, "0.012"),
        (123, "0.123"),
        (1000, "1.000"),
        (1001, "1.001"),
        (1012, "1.012"),
        (1123, "1.123"),
        (10000, "10.000"),
        (10001, "10.001"),
        (10012, "10.012"),
        (10123, "10.123"),
        (60000, "1:00.000"),
        (3_600_000, "1:0:00.000"),
    ];

    for (millis, readable) in pairs {
        assert_eq!(millis.to_readable_string(), readable.to_string());
    }
}
