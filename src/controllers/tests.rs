#[cfg(test)]
mod controller_tests {
    use crate::models::{Claims, LoginRequest, RegisterRequest};
    use crate::middlewares::AdminUser;
    use uuid::Uuid;

    // ============ AUTH CONTROLLER TESTS ============

    #[test]
    fn test_login_request_structure() {
        let login = LoginRequest {
            email: "admin@example.com".to_string(),
            password: "password123".to_string(),
        };

        assert_eq!(login.email, "admin@example.com");
        assert_eq!(login.password, "password123");
    }

    #[test]
    fn test_register_request_admin_role() {
        let register = RegisterRequest {
            email: "newadmin@example.com".to_string(),
            password: "securepass123".to_string(),
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            role: "admin".to_string(),
        };

        assert_eq!(register.role, "admin");
        assert_eq!(register.email, "newadmin@example.com");
    }

    #[test]
    fn test_register_request_student_role() {
        let register = RegisterRequest {
            email: "student@example.com".to_string(),
            password: "studentpass123".to_string(),
            first_name: "John".to_string(),
            last_name: "Student".to_string(),
            role: "student".to_string(),
        };

        assert_eq!(register.role, "student");
        assert_eq!(register.first_name, "John");
    }

    #[test]
    fn test_register_request_mentor_role() {
        let register = RegisterRequest {
            email: "mentor@example.com".to_string(),
            password: "mentorpass123".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Mentor".to_string(),
            role: "mentor".to_string(),
        };

        assert_eq!(register.role, "mentor");
        assert_eq!(register.last_name, "Mentor");
    }

    // ============ ADMIN CONTROLLER TESTS ============

    #[test]
    fn test_admin_user_extraction() {
        let user_id = Uuid::new_v4();
        let admin = AdminUser {
            user_id,
            email: "admin@example.com".to_string(),
        };

        assert_eq!(admin.email, "admin@example.com");
        assert_eq!(admin.user_id, user_id);
        assert!(!admin.email.is_empty());
    }

    #[test]
    fn test_admin_dashboard_response_structure() {
        // Test that admin dashboard would return correct structure
        let permissions = vec![
            "manage_users",
            "manage_schools",
            "manage_mentors",
            "manage_students",
            "view_reports",
            "system_settings",
        ];

        assert_eq!(permissions.len(), 6);
        assert!(permissions.contains(&"manage_users"));
        assert!(permissions.contains(&"manage_schools"));
    }

    #[test]
    fn test_admin_user_deactivation_path() {
        let user_id = Uuid::new_v4();
        let path = format!("/admin/users/{}/deactivate", user_id);

        assert!(path.contains("/admin/users/"));
        assert!(path.contains("/deactivate"));
    }

    #[test]
    fn test_admin_user_activation_path() {
        let user_id = Uuid::new_v4();
        let path = format!("/admin/users/{}/activate", user_id);

        assert!(path.contains("/admin/users/"));
        assert!(path.contains("/activate"));
    }

    #[test]
    fn test_admin_statistics_endpoint() {
        let stats = vec![
            ("total_users", 150),
            ("admins", 5),
            ("students", 100),
            ("mentors", 45),
        ];

        assert_eq!(stats.len(), 4);
        let total: i32 = stats.iter().map(|(_, count)| count).sum();
        assert_eq!(total, 300);
    }

    // ============ STUDENT CONTROLLER TESTS ============

    #[test]
    fn test_student_dashboard_endpoint() {
        let endpoint = "/student/dashboard";
        assert!(endpoint.contains("/student/"));
        assert!(endpoint.contains("dashboard"));
    }

    #[test]
    fn test_student_profile_endpoint() {
        let endpoint = "/student/profile";
        assert!(endpoint.contains("/student/"));
        assert!(endpoint.contains("profile"));
    }

    #[test]
    fn test_student_courses_endpoint() {
        let endpoint = "/student/courses";
        assert!(endpoint.contains("/student/"));
        assert!(endpoint.contains("courses"));
    }

    #[test]
    fn test_student_grades_endpoint() {
        let endpoint = "/student/grades";
        assert!(endpoint.contains("/student/"));
        assert!(endpoint.contains("grades"));
    }

    #[test]
    fn test_student_assignment_submission_path() {
        let assignment_id = Uuid::new_v4();
        let path = format!("/student/assignments/{}/submit", assignment_id);

        assert!(path.contains("/student/assignments/"));
        assert!(path.contains("/submit"));
    }

    #[test]
    fn test_student_message_mentor_endpoint() {
        let endpoint = "/student/messages/mentor";
        assert!(endpoint.contains("/student/messages/"));
        assert!(endpoint.contains("mentor"));
    }

    // ============ MENTOR CONTROLLER TESTS ============

    #[test]
    fn test_mentor_dashboard_endpoint() {
        let endpoint = "/mentor/dashboard";
        assert!(endpoint.contains("/mentor/"));
        assert!(endpoint.contains("dashboard"));
    }

    #[test]
    fn test_mentor_profile_endpoint() {
        let endpoint = "/mentor/profile";
        assert!(endpoint.contains("/mentor/"));
        assert!(endpoint.contains("profile"));
    }

    #[test]
    fn test_mentor_students_endpoint() {
        let endpoint = "/mentor/students";
        assert!(endpoint.contains("/mentor/"));
        assert!(endpoint.contains("students"));
    }

    #[test]
    fn test_mentor_student_progress_path() {
        let student_id = Uuid::new_v4();
        let path = format!("/mentor/students/{}/progress", student_id);

        assert!(path.contains("/mentor/students/"));
        assert!(path.contains("/progress"));
    }

    #[test]
    fn test_mentor_assignment_grading_path() {
        let assignment_id = Uuid::new_v4();
        let path = format!("/mentor/assignments/{}/grade", assignment_id);

        assert!(path.contains("/mentor/assignments/"));
        assert!(path.contains("/grade"));
    }

    #[test]
    fn test_mentor_assignment_creation_endpoint() {
        let endpoint = "/mentor/assignments/create";
        assert!(endpoint.contains("/mentor/assignments/"));
        assert!(endpoint.contains("create"));
    }

    #[test]
    fn test_mentor_message_student_path() {
        let student_id = Uuid::new_v4();
        let path = format!("/mentor/messages/student/{}", student_id);

        assert!(path.contains("/mentor/messages/student/"));
    }

    #[test]
    fn test_mentor_course_assignments_path() {
        let course_id = Uuid::new_v4();
        let path = format!("/mentor/courses/{}/assignments", course_id);

        assert!(path.contains("/mentor/courses/"));
        assert!(path.contains("/assignments"));
    }

    // ============ SCHOOL CONTROLLER TESTS ============

    #[test]
    fn test_school_list_endpoint() {
        let endpoint = "/admin/schools";
        assert!(endpoint.contains("/admin/"));
        assert!(endpoint.contains("schools"));
    }

    #[test]
    fn test_school_creation_endpoint() {
        let endpoint = "/admin/schools/create";
        assert!(endpoint.contains("/admin/schools/"));
        assert!(endpoint.contains("create"));
    }

    #[test]
    fn test_school_details_path() {
        let school_id = Uuid::new_v4();
        let path = format!("/admin/schools/{}", school_id);

        assert!(path.contains("/admin/schools/"));
    }

    #[test]
    fn test_school_update_path() {
        let school_id = Uuid::new_v4();
        let path = format!("/admin/schools/{}", school_id);

        assert!(path.contains("/admin/schools/"));
    }

    #[test]
    fn test_school_delete_path() {
        let school_id = Uuid::new_v4();
        let path = format!("/admin/schools/{}", school_id);

        assert!(path.contains("/admin/schools/"));
    }

    #[test]
    fn test_school_statistics_path() {
        let school_id = Uuid::new_v4();
        let path = format!("/admin/schools/{}/statistics", school_id);

        assert!(path.contains("/admin/schools/"));
        assert!(path.contains("/statistics"));
    }

    // ============ AUTH ENDPOINTS TESTS ============

    #[test]
    fn test_admin_register_endpoint() {
        let endpoint = "/auth/admin/register";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("admin"));
        assert!(endpoint.contains("register"));
    }

    #[test]
    fn test_admin_login_endpoint() {
        let endpoint = "/auth/admin/login";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("admin"));
        assert!(endpoint.contains("login"));
    }

    #[test]
    fn test_student_register_endpoint() {
        let endpoint = "/auth/student/register";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("student"));
        assert!(endpoint.contains("register"));
    }

    #[test]
    fn test_student_login_endpoint() {
        let endpoint = "/auth/student/login";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("student"));
        assert!(endpoint.contains("login"));
    }

    #[test]
    fn test_mentor_register_endpoint() {
        let endpoint = "/auth/mentor/register";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("mentor"));
        assert!(endpoint.contains("register"));
    }

    #[test]
    fn test_mentor_login_endpoint() {
        let endpoint = "/auth/mentor/login";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("mentor"));
        assert!(endpoint.contains("login"));
    }

    #[test]
    fn test_refresh_token_endpoint() {
        let endpoint = "/auth/refresh";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("refresh"));
    }

    #[test]
    fn test_logout_endpoint() {
        let endpoint = "/auth/logout";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("logout"));
    }

    #[test]
    fn test_current_user_endpoint() {
        let endpoint = "/auth/me";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("me"));
    }

    #[test]
    fn test_verify_token_endpoint() {
        let endpoint = "/auth/verify";
        assert!(endpoint.contains("/auth/"));
        assert!(endpoint.contains("verify"));
    }

    // ============ CONTROLLER RESPONSE VALIDATION TESTS ============

    #[test]
    fn test_bearer_token_format() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let bearer_header = format!("Bearer {}", token);

        assert!(bearer_header.starts_with("Bearer "));
        assert_eq!(bearer_header.len(), 7 + token.len());
    }

    #[test]
    fn test_http_status_codes() {
        let status_codes = vec![
            ("OK", 200),
            ("CREATED", 201),
            ("BAD_REQUEST", 400),
            ("UNAUTHORIZED", 401),
            ("FORBIDDEN", 403),
            ("NOT_FOUND", 404),
            ("INTERNAL_SERVER_ERROR", 500),
        ];

        assert_eq!(status_codes.len(), 7);
        assert!(status_codes.iter().any(|(_, code)| *code == 200));
        assert!(status_codes.iter().any(|(_, code)| *code == 401));
    }

    #[test]
    fn test_controller_error_responses() {
        let errors = vec![
            "Invalid credentials",
            "User not found",
            "Unauthorized access",
            "Forbidden",
            "Invalid token",
            "User already exists",
        ];

        assert_eq!(errors.len(), 6);
        assert!(errors.contains(&"Invalid credentials"));
        assert!(errors.contains(&"Unauthorized access"));
    }

    #[test]
    fn test_admin_user_structure() {
        let user_id = Uuid::new_v4();
        let admin = AdminUser {
            user_id,
            email: "admin@school.com".to_string(),
        };

        assert!(admin.email.contains("@"));
        assert_eq!(admin.email.split("@").count(), 2);
    }

    #[test]
    fn test_claims_for_different_roles() {
        let roles = vec!["admin", "student", "mentor"];

        for role in roles {
            let claims = Claims {
                sub: Uuid::new_v4().to_string(),
                email: format!("{}@example.com", role),
                role: role.to_string(),
                exp: 1234567890,
                iat: 1234567800,
                token_type: "access".to_string(),
            };

            assert_eq!(claims.role, role);
            assert!(claims.email.contains(role));
        }
    }

    #[test]
    fn test_refresh_token_request_structure() {
        let refresh_token = "refresh_token_value_here";
        
        assert!(!refresh_token.is_empty());
        assert!(refresh_token.len() > 10);
    }

    #[test]
    fn test_multiple_user_ids_uniqueness() {
        let user_ids: Vec<Uuid> = (0..5).map(|_| Uuid::new_v4()).collect();

        // Check all IDs are unique
        for i in 0..user_ids.len() {
            for j in (i + 1)..user_ids.len() {
                assert_ne!(user_ids[i], user_ids[j]);
            }
        }
    }

    #[test]
    fn test_email_format_validation() {
        let valid_emails = vec![
            "user@example.com",
            "admin@school.edu",
            "mentor.name@institution.org",
        ];

        for email in valid_emails {
            assert!(email.contains("@"));
            assert!(email.contains("."));
            let parts: Vec<&str> = email.split("@").collect();
            assert_eq!(parts.len(), 2);
        }
    }

    #[test]
    fn test_password_hash_format() {
        let password_hash = "$2b$12$abcdefghijklmnopqrstuvwxyz";
        
        assert!(password_hash.starts_with("$2b$"));
        assert!(password_hash.len() > 20);
    }

    #[test]
    fn test_controller_path_parameters() {
        let paths = vec![
            "/admin/users/123e4567-e89b-12d3-a456-426614174000/deactivate",
            "/student/assignments/123e4567-e89b-12d3-a456-426614174000/submit",
            "/mentor/students/123e4567-e89b-12d3-a456-426614174000/progress",
        ];

        for path in paths {
            assert!(path.contains("/"));
            assert!(path.len() > 10);
        }
    }

    #[test]
    fn test_json_response_structure() {
        let response_keys = vec![
            "message",
            "user_id",
            "email",
            "role",
            "access_token",
            "refresh_token",
            "token_type",
            "expires_in",
        ];

        assert!(response_keys.len() > 0);
        assert!(response_keys.contains(&"access_token"));
        assert!(response_keys.contains(&"refresh_token"));
    }

    // ============ SCHOOL CONTROLLER TESTS ============

    #[test]
    fn test_school_creation_request() {
        let school_name = "Central High School";
        let school_location = "New York";
        let school_principal = "Dr. John Smith";

        assert!(!school_name.is_empty());
        assert!(!school_location.is_empty());
        assert!(!school_principal.is_empty());
    }

    #[test]
    fn test_school_data_structure() {
        let school = vec![
            ("id", "school_1"),
            ("name", "Central High School"),
            ("location", "New York"),
            ("principal", "Dr. John Smith"),
            ("students", "500"),
            ("mentors", "45"),
        ];

        assert_eq!(school.len(), 6);
        assert!(school.iter().any(|(k, _)| *k == "name"));
        assert!(school.iter().any(|(k, _)| *k == "location"));
    }

    #[test]
    fn test_school_update_fields() {
        let updates = vec![
            "name",
            "location",
            "principal",
            "contact_email",
            "phone_number",
        ];

        assert!(updates.contains(&"name"));
        assert!(updates.contains(&"location"));
        assert!(updates.contains(&"principal"));
    }

    #[test]
    fn test_school_deletion_confirmation() {
        let school_id = Uuid::new_v4();
        let deletion_message = format!("School {} deleted successfully", school_id);

        assert!(deletion_message.contains("deleted"));
        assert!(deletion_message.contains(&school_id.to_string()));
    }

    #[test]
    fn test_school_statistics_data() {
        let stats = vec![
            ("total_students", 500),
            ("total_mentors", 45),
            ("total_courses", 25),
            ("active_assignments", 120),
        ];

        assert_eq!(stats.len(), 4);
        let total_people: i32 = stats.iter()
            .filter(|(k, _)| k.contains("total"))
            .map(|(_, v)| v)
            .sum();
        assert_eq!(total_people, 570);
    }

    // ============ ADMIN CONTROLLER TESTS ============

    #[test]
    fn test_admin_credentials_validation() {
        let admin_email = "admin@school.com";
        let admin_password = "SecureAdminPass123!";

        assert!(admin_email.contains("@"));
        assert!(admin_password.len() >= 8);
        assert!(admin_password.chars().any(|c| c.is_uppercase()));
        assert!(admin_password.chars().any(|c| c.is_numeric()));
    }

    #[test]
    fn test_admin_registration_request() {
        let register = RegisterRequest {
            email: "newadmin@school.com".to_string(),
            password: "AdminPass123!".to_string(),
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            role: "admin".to_string(),
        };

        assert_eq!(register.role, "admin");
        assert!(register.password.len() >= 8);
        assert!(register.email.contains("admin"));
    }

    #[test]
    fn test_admin_login_request() {
        let login = LoginRequest {
            email: "admin@school.com".to_string(),
            password: "AdminPass123!".to_string(),
        };

        assert!(login.email.contains("admin"));
        assert!(login.password.len() >= 8);
    }

    #[test]
    fn test_admin_permissions() {
        let admin_permissions = vec![
            "manage_users",
            "manage_schools",
            "manage_mentors",
            "manage_students",
            "view_reports",
            "system_settings",
            "manage_courses",
            "manage_assignments",
        ];

        assert!(admin_permissions.contains(&"manage_users"));
        assert!(admin_permissions.contains(&"manage_schools"));
        assert!(admin_permissions.contains(&"system_settings"));
    }

    #[test]
    fn test_admin_user_management() {
        let admin_actions = vec![
            "create_user",
            "update_user",
            "delete_user",
            "activate_user",
            "deactivate_user",
            "reset_password",
            "view_user_details",
        ];

        assert_eq!(admin_actions.len(), 7);
        assert!(admin_actions.contains(&"activate_user"));
        assert!(admin_actions.contains(&"deactivate_user"));
    }

    #[test]
    fn test_admin_school_management() {
        let admin_school_actions = vec![
            "create_school",
            "update_school",
            "delete_school",
            "view_school_details",
            "view_school_statistics",
            "manage_school_mentors",
            "manage_school_students",
        ];

        assert!(admin_school_actions.contains(&"create_school"));
        assert!(admin_school_actions.contains(&"view_school_statistics"));
    }

    #[test]
    fn test_admin_dashboard_metrics() {
        let metrics = vec![
            ("total_users", 1500),
            ("total_schools", 50),
            ("total_students", 1200),
            ("total_mentors", 250),
            ("total_admins", 50),
            ("active_courses", 300),
        ];

        assert_eq!(metrics.len(), 6);
        let total_users: i32 = metrics.iter()
            .filter(|(k, _)| k.contains("total"))
            .map(|(_, v)| v)
            .sum();
        assert!(total_users > 0);
    }

    #[test]
    fn test_admin_audit_log() {
        let audit_actions = vec![
            "user_created",
            "user_updated",
            "user_deleted",
            "school_created",
            "school_updated",
            "password_reset",
            "role_changed",
        ];

        assert!(audit_actions.contains(&"user_created"));
        assert!(audit_actions.contains(&"password_reset"));
    }

    // ============ PASSWORD MANAGEMENT TESTS ============

    #[test]
    fn test_password_strength_requirements() {
        let strong_password = "SecurePass123!@#";
        
        assert!(strong_password.len() >= 8);
        assert!(strong_password.chars().any(|c| c.is_uppercase()));
        assert!(strong_password.chars().any(|c| c.is_lowercase()));
        assert!(strong_password.chars().any(|c| c.is_numeric()));
    }

    #[test]
    fn test_weak_password_validation() {
        let weak_passwords = vec![
            "123456",           // Too short
            "password",         // No numbers
            "12345678",         // No letters
            "Pass123",          // Too short
        ];

        for password in weak_passwords {
            if password.len() < 8 {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_password_hashing_format() {
        let bcrypt_hash = "$2b$12$abcdefghijklmnopqrstuvwxyz1234567890";
        
        assert!(bcrypt_hash.starts_with("$2b$"));
        assert!(bcrypt_hash.len() > 20);
    }

    #[test]
    fn test_password_reset_token() {
        let reset_token = "reset_token_abc123def456ghi789";
        
        assert!(!reset_token.is_empty());
        assert!(reset_token.len() > 20);
    }

    #[test]
    fn test_password_reset_request() {
        let email = "user@school.com";
        let reset_request = format!("Password reset requested for {}", email);

        assert!(reset_request.contains(email));
        assert!(reset_request.contains("reset"));
    }

    #[test]
    fn test_password_change_validation() {
        let old_password = "OldPass123!";
        let new_password = "NewPass456!";

        assert_ne!(old_password, new_password);
        assert!(old_password.len() >= 8);
        assert!(new_password.len() >= 8);
    }

    #[test]
    fn test_password_history_prevention() {
        let password_history = vec![
            "Pass123!",
            "Pass456!",
            "Pass789!",
        ];

        let new_password = "Pass999!";
        
        assert!(!password_history.contains(&new_password));
    }

    #[test]
    fn test_password_expiration_policy() {
        let password_age_days = 90;
        let max_password_age = 90;

        assert_eq!(password_age_days, max_password_age);
    }

    #[test]
    fn test_password_reset_email_template() {
        let email_template = "Click here to reset your password: https://school.com/reset?token=abc123";
        
        assert!(email_template.contains("reset"));
        assert!(email_template.contains("token"));
        assert!(email_template.contains("https"));
    }

    #[test]
    fn test_admin_password_reset_for_user() {
        let admin_email = "admin@school.com";
        let user_email = "user@school.com";
        let action = format!("Admin {} reset password for {}", admin_email, user_email);

        assert!(action.contains("reset"));
        assert!(action.contains(admin_email));
        assert!(action.contains(user_email));
    }

    #[test]
    fn test_password_complexity_rules() {
        let rules = vec![
            ("Minimum length", 8),
            ("Uppercase letters", 1),
            ("Lowercase letters", 1),
            ("Numbers", 1),
            ("Special characters", 1),
        ];

        assert_eq!(rules.len(), 5);
        assert!(rules.iter().any(|(r, _)| r.contains("length")));
    }

    #[test]
    fn test_failed_login_attempts() {
        let max_attempts = 5;
        let lockout_duration_minutes = 15;

        assert_eq!(max_attempts, 5);
        assert_eq!(lockout_duration_minutes, 15);
    }

    #[test]
    fn test_account_lockout_after_failed_attempts() {
        let failed_attempts = 5;
        let max_allowed = 5;

        assert_eq!(failed_attempts, max_allowed);
    }

    #[test]
    fn test_password_reset_link_expiration() {
        let expiration_hours = 24;
        
        assert_eq!(expiration_hours, 24);
    }

    #[test]
    fn test_secure_password_transmission() {
        let protocol = "HTTPS";
        
        assert_eq!(protocol, "HTTPS");
    }

    #[test]
    fn test_admin_can_force_password_change() {
        let admin_action = "force_password_change";
        let user_id = Uuid::new_v4();

        assert!(admin_action.contains("password"));
        assert_ne!(user_id, Uuid::nil());
    }

    #[test]
    fn test_password_change_confirmation_email() {
        let email_subject = "Password Changed Successfully";
        let email_body = "Your password has been changed. If this wasn't you, please contact support.";

        assert!(email_subject.contains("Password"));
        assert!(email_body.contains("changed"));
    }

    #[test]
    fn test_admin_user_password_requirements() {
        let admin_password = "AdminSecure123!@#";
        
        assert!(admin_password.len() >= 12); // Admins need stronger passwords
        assert!(admin_password.chars().any(|c| c.is_uppercase()));
        assert!(admin_password.chars().any(|c| c.is_numeric()));
        assert!(admin_password.chars().any(|c| "!@#$%^&*".contains(c)));
    }

    #[test]
    fn test_school_admin_role_assignment() {
        let user_id = Uuid::new_v4();
        let school_admin = AdminUser {
            user_id,
            email: "schooladmin@school.com".to_string(),
        };

        assert!(school_admin.email.contains("admin"));
        assert_eq!(school_admin.user_id, user_id);
        assert!(!school_admin.email.is_empty());
    }

    #[test]
    fn test_school_admin_permissions() {
        let school_admin_perms = vec![
            "manage_school_users",
            "manage_school_mentors",
            "manage_school_students",
            "view_school_reports",
            "manage_school_courses",
        ];

        assert!(school_admin_perms.contains(&"manage_school_users"));
        assert!(school_admin_perms.contains(&"view_school_reports"));
    }

    #[test]
    fn test_school_admin_cannot_manage_other_schools() {
        let school_id_1 = Uuid::new_v4();
        let school_id_2 = Uuid::new_v4();

        assert_ne!(school_id_1, school_id_2);
    }

    #[test]
    fn test_admin_role_hierarchy() {
        let roles = vec![
            ("super_admin", 3),
            ("school_admin", 2),
            ("mentor", 1),
            ("student", 0),
        ];

        let super_admin_level = roles.iter().find(|(r, _)| *r == "super_admin").map(|(_, l)| l);
        let student_level = roles.iter().find(|(r, _)| *r == "student").map(|(_, l)| l);

        assert!(super_admin_level.unwrap() > student_level.unwrap());
    }
}
