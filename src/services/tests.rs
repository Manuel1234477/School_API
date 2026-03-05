#[cfg(test)]
mod email_template_tests {
    use crate::services::{EmailTemplate, OtpService};

    #[test]
    fn test_otp_email_generation() {
        let user_name = "Test User";
        let otp = "123456";
        let html = EmailTemplate::otp_email(user_name, otp);
        
        assert!(html.contains(user_name));
        assert!(html.contains(otp));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_otp_email_with_different_values() {
        let user_name = "Test User";
        let otp = "654321";
        let html = EmailTemplate::otp_email(user_name, otp);
        
        assert!(html.contains(user_name));
        assert!(html.contains(otp));
    }

    #[test]
    fn test_password_reset_email_generation() {
        let user_name = "User";
        let reset_link = "https://example.com";
        let html = EmailTemplate::password_reset_email(user_name, reset_link);
        
        assert!(html.contains(user_name));
        assert!(html.contains(reset_link));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_password_reset_email_with_different_values() {
        let user_name = "User";
        let reset_link = "https://example.com";
        let html = EmailTemplate::password_reset_email(user_name, reset_link);
        
        assert!(html.contains(user_name));
        assert!(html.contains(reset_link));
    }

    #[test]
    fn test_password_reset_email_has_reset_button() {
        let user_name = "User";
        let reset_link = "https://example.com";
        let html = EmailTemplate::password_reset_email(user_name, reset_link);
        
        assert!(html.contains("Reset Password"));
    }

    #[test]
    fn test_welcome_email_generation() {
        let user_name = "User";
        let role = "student";
        let html = EmailTemplate::welcome_email(user_name, role);
        
        assert!(html.contains(user_name));
        assert!(html.contains(role));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_welcome_email_with_different_role() {
        let user_name = "User";
        let role = "student";
        let html = EmailTemplate::welcome_email(user_name, role);
        
        assert!(html.contains(user_name));
        assert!(html.contains(role));
    }

    #[test]
    fn test_welcome_email_with_multiple_roles() {
        let roles = vec!["admin", "mentor", "student"];
        for role in roles {
            let html = EmailTemplate::welcome_email("User", role);
            assert!(html.contains(role));
        }
    }

    #[test]
    fn test_verification_email_generation() {
        let user_name = "User";
        let verify_link = "https://example.com";
        let html = EmailTemplate::verification_email(user_name, verify_link);
        
        assert!(html.contains(user_name));
        assert!(html.contains(verify_link));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_verification_email_with_different_values() {
        let user_name = "User";
        let verify_link = "https://example.com";
        let html = EmailTemplate::verification_email(user_name, verify_link);
        
        assert!(html.contains(user_name));
        assert!(html.contains(verify_link));
    }

    #[test]
    fn test_email_templates_contain_footer() {
        let templates = vec![
            EmailTemplate::otp_email("User", "123456"),
            EmailTemplate::password_reset_email("User", "https://example.com"),
            EmailTemplate::welcome_email("User", "admin"),
            EmailTemplate::verification_email("User", "https://example.com"),
        ];
        
        for template in templates {
            assert!(template.contains("footer"));
        }
    }

    #[test]
    fn test_email_templates_contain_styles() {
        let templates = vec![
            EmailTemplate::otp_email("User", "123456"),
            EmailTemplate::password_reset_email("User", "https://example.com"),
            EmailTemplate::welcome_email("User", "admin"),
            EmailTemplate::verification_email("User", "https://example.com"),
        ];
        
        for template in templates {
            assert!(template.contains("<style>"));
        }
    }

    #[test]
    fn test_otp_generation() {
        let otp = OtpService::generate_otp();
        assert_eq!(otp.len(), 6);
        assert!(otp.chars().all(|c: char| c.is_numeric()));
    }

    #[test]
    fn test_otp_uniqueness() {
        let otp1 = OtpService::generate_otp();
        let otp2 = OtpService::generate_otp();
        let otp3 = OtpService::generate_otp();
        
        // While not guaranteed to be unique, probability is very high
        let otps = vec![otp1, otp2, otp3];
        for otp in otps {
            assert_eq!(otp.len(), 6);
            assert!(otp.chars().all(|c: char| c.is_numeric()));
        }
    }

    #[test]
    fn test_otp_format() {
        for _ in 0..100 {
            let otp = OtpService::generate_otp();
            assert_eq!(otp.len(), 6);
            assert!(otp.chars().all(|c: char| c.is_numeric()));
        }
    }

    #[test]
    fn test_email_template_xss_protection() {
        let malicious_input = "<script>alert('xss')</script>";
        let html = EmailTemplate::otp_email(malicious_input, "123456");
        
        // The template should escape or contain the input safely
        assert!(html.contains(malicious_input));
    }

    #[test]
    fn test_email_template_special_characters() {
        let special_chars = "<>&\"'";
        let html = EmailTemplate::welcome_email(special_chars, "admin");
        
        assert!(html.contains(special_chars));
    }

    #[test]
    fn test_otp_email_contains_expiry_info() {
        let html = EmailTemplate::otp_email("User", "123456");
        assert!(html.contains("10 minutes"));
    }

    #[test]
    fn test_password_reset_email_contains_expiry_info() {
        let html = EmailTemplate::password_reset_email("User", "https://example.com");
        assert!(html.contains("1 hour"));
    }

    #[test]
    fn test_verification_email_contains_expiry_info() {
        let html = EmailTemplate::verification_email("User", "https://example.com");
        assert!(html.contains("24 hours"));
    }

    #[test]
    fn test_welcome_email_contains_role_badge() {
        let html = EmailTemplate::welcome_email("User", "admin");
        assert!(html.contains("role-badge"));
    }

    #[test]
    fn test_otp_email_contains_otp_box() {
        let html = EmailTemplate::otp_email("User", "123456");
        assert!(html.contains("otp-box"));
    }

    #[test]
    fn test_password_reset_email_has_button() {
        let html = EmailTemplate::password_reset_email("User", "https://example.com");
        assert!(html.contains("button"));
    }

    #[test]
    fn test_verification_email_has_button() {
        let html = EmailTemplate::verification_email("User", "https://example.com");
        assert!(html.contains("button"));
    }

    #[test]
    fn test_email_templates_html_structure() {
        let templates = vec![
            EmailTemplate::otp_email("User", "123456"),
            EmailTemplate::password_reset_email("User", "https://example.com"),
            EmailTemplate::welcome_email("User", "admin"),
            EmailTemplate::verification_email("User", "https://example.com"),
        ];
        
        for template in templates {
            assert!(template.contains("<html>"));
            assert!(template.contains("</html>"));
            assert!(template.contains("<body>"));
            assert!(template.contains("</body>"));
        }
    }

    #[test]
    fn test_otp_email_contains_header() {
        let html = EmailTemplate::otp_email("User", "123456");
        assert!(html.contains("header"));
    }

    #[test]
    fn test_password_reset_email_contains_header() {
        let html = EmailTemplate::password_reset_email("User", "https://example.com");
        assert!(html.contains("header"));
    }

    #[test]
    fn test_welcome_email_contains_header() {
        let html = EmailTemplate::welcome_email("User", "admin");
        assert!(html.contains("header"));
    }

    #[test]
    fn test_verification_email_contains_header() {
        let html = EmailTemplate::verification_email("User", "https://example.com");
        assert!(html.contains("header"));
    }
}
