use crate::config::Role;
use crate::config::Config;

// Check permission with function
pub fn check_permission(config: &Config, perm: String, role: &Role) -> bool {
    for r in &role.perms {
        if r == &perm {
            // Return true if the permission is set
            return true;
        } else if r == &format!("-{}", perm) {
            // Return false if permission is explicitly denied with a dash
            return false;
        } 
    } 
    // Check if the role inherits any roles
    if !role.inherit.is_empty() {
        return check_permission(config, perm, &config.roles.get(&role.inherit).unwrap());
    } else {
        // Check if the perm starts with response. 
        if perm.starts_with("response.") {
            // Default to true if the command is a canned response, can still be negated with -response.example
            return true;
        } else {
            return false;
        }
    }
}
