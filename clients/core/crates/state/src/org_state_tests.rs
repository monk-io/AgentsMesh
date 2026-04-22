use agentsmesh_types::{OrgMemberView, Organization};

use crate::org_state::OrgState;

fn make_org(id: i64, name: &str) -> Organization {
    Organization { id, name: name.to_string(), slug: name.to_lowercase(), role: None, logo_url: None, subscription_plan: None, subscription_status: None }
}

fn make_member(id: i64, role: &str) -> OrgMemberView {
    OrgMemberView { id, user_id: id + 100, username: format!("user{id}"), email: None, name: None, avatar_url: None, role: role.to_string(), joined_at: None }
}

#[test]
fn new_is_empty() {
    let s = OrgState::new();
    assert!(s.organizations().is_empty());
    assert!(s.current_org().is_none());
    assert!(s.members().is_empty());
}

#[test]
fn set_and_get_organizations() {
    let mut s = OrgState::new();
    s.set_organizations(vec![make_org(1, "Alpha"), make_org(2, "Beta")]);
    assert_eq!(s.organizations().len(), 2);
    assert_eq!(s.organizations()[0].name, "Alpha");
}

#[test]
fn set_organizations_replaces_previous() {
    let mut s = OrgState::new();
    s.set_organizations(vec![make_org(1, "Alpha")]);
    s.set_organizations(vec![make_org(2, "Beta"), make_org(3, "Gamma")]);
    assert_eq!(s.organizations().len(), 2);
    assert_eq!(s.organizations()[0].id, 2);
}

#[test]
fn set_current_org() {
    let mut s = OrgState::new();
    s.set_current_org(Some(make_org(1, "Alpha")));
    assert_eq!(s.current_org().unwrap().id, 1);
    s.set_current_org(None);
    assert!(s.current_org().is_none());
}

#[test]
fn set_members() {
    let mut s = OrgState::new();
    s.set_members(vec![make_member(1, "admin"), make_member(2, "member")]);
    assert_eq!(s.members().len(), 2);
}

#[test]
fn add_member() {
    let mut s = OrgState::new();
    s.add_member(make_member(1, "admin"));
    s.add_member(make_member(2, "member"));
    assert_eq!(s.members().len(), 2);
    assert_eq!(s.members()[1].role, "member");
}

#[test]
fn remove_member() {
    let mut s = OrgState::new();
    s.set_members(vec![make_member(1, "admin"), make_member(2, "member")]);
    s.remove_member("1");
    assert_eq!(s.members().len(), 1);
    assert_eq!(s.members()[0].id, 2);
}

#[test]
fn remove_member_nonexistent_is_noop() {
    let mut s = OrgState::new();
    s.set_members(vec![make_member(1, "admin")]);
    s.remove_member("999");
    assert_eq!(s.members().len(), 1);
}
