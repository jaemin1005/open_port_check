use std::collections::HashSet;

pub fn remove_duplicates(ports: Vec<(String, String, String)>) -> Vec<(String, String, String)> {
    let mut unique_ports: HashSet<(String, String, String)> = HashSet::new();

    // HashSet을 이용해 중복 제거
    ports.into_iter().for_each(|port| {
        unique_ports.insert(port);
    });

    unique_ports.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    // 중복된 값이 있을 때
    #[test]
    fn test_remove_duplicates_with_duplicates() {
        let ports = vec![
            (
                "8080".to_string(),
                "process1".to_string(),
                "1234".to_string(),
            ),
            (
                "8080".to_string(),
                "process1".to_string(),
                "1234".to_string(),
            ),
            (
                "3000".to_string(),
                "process2".to_string(),
                "5678".to_string(),
            ),
        ];

        let result = remove_duplicates(ports);

        let expected = vec![
            (
                "8080".to_string(),
                "process1".to_string(),
                "1234".to_string(),
            ),
            (
                "3000".to_string(),
                "process2".to_string(),
                "5678".to_string(),
            ),
        ];

        assert_eq!(result.len(), 2);
        assert!(result.contains(&expected[0]));
        assert!(result.contains(&expected[1]));
    }

    // 중복된 갑이 존재하지 않을 때
    #[test]
    fn test_remove_duplicates_without_duplicates() {
        let ports = vec![
            (
                "8080".to_string(),
                "process1".to_string(),
                "1234".to_string(),
            ),
            (
                "3000".to_string(),
                "process2".to_string(),
                "5678".to_string(),
            ),
        ];

        let result = remove_duplicates(ports.clone());

        assert_eq!(result.len(), ports.len());
        assert_eq!(result, ports);
    }

    // 값이 비어있는 경우
    #[test]
    fn test_remove_duplicates_empty_input() {
        let ports: Vec<(String, String, String)> = vec![];

        let result = remove_duplicates(ports.clone());

        assert!(result.is_empty());
    }
}
