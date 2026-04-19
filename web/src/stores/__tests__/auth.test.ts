import { describe, it, expect, beforeEach } from 'vitest'
import { useAuthStore } from '../auth'

describe('useAuthStore', () => {
  beforeEach(() => {
    useAuthStore.setState({
      user: null,
      currentOrg: null,
      organizations: [],
    })
  })

  describe('initial state', () => {
    it('should have null user', () => {
      expect(useAuthStore.getState().user).toBeNull()
    })

    it('should have null currentOrg', () => {
      expect(useAuthStore.getState().currentOrg).toBeNull()
    })

    it('should have empty organizations', () => {
      expect(useAuthStore.getState().organizations).toEqual([])
    })
  })

  describe('setAuth', () => {
    it('should set user', () => {
      const user = { id: 1, email: 'test@example.com', username: 'testuser', name: 'Test User' }
      useAuthStore.getState().setAuth('test-token', user)
      expect(useAuthStore.getState().user).toEqual(user)
    })

    it('should handle user without optional fields', () => {
      const user = { id: 1, email: 'test@example.com', username: 'testuser' }
      useAuthStore.getState().setAuth('test-token', user)
      expect(useAuthStore.getState().user).toEqual(user)
      expect(useAuthStore.getState().user?.name).toBeUndefined()
    })
  })

  describe('setOrganizations', () => {
    it('should set organizations', () => {
      const orgs = [
        { id: 1, name: 'Org 1', slug: 'org-1', role: 'owner' },
        { id: 2, name: 'Org 2', slug: 'org-2', role: 'member' },
      ]
      useAuthStore.getState().setOrganizations(orgs)
      expect(useAuthStore.getState().organizations).toEqual(orgs)
    })

    it('should auto-select first org if none selected', () => {
      const orgs = [
        { id: 1, name: 'Org 1', slug: 'org-1', role: 'owner' },
        { id: 2, name: 'Org 2', slug: 'org-2', role: 'member' },
      ]
      useAuthStore.getState().setOrganizations(orgs)
      expect(useAuthStore.getState().currentOrg).toEqual(orgs[0])
    })

    it('should not change currentOrg if already selected', () => {
      const existingOrg = { id: 3, name: 'Existing', slug: 'existing', role: 'admin' }
      useAuthStore.setState({ currentOrg: existingOrg })
      const orgs = [
        { id: 1, name: 'Org 1', slug: 'org-1', role: 'owner' },
        { id: 2, name: 'Org 2', slug: 'org-2', role: 'member' },
      ]
      useAuthStore.getState().setOrganizations(orgs)
      expect(useAuthStore.getState().currentOrg).toEqual(existingOrg)
    })

    it('should handle empty organizations array', () => {
      useAuthStore.getState().setOrganizations([])
      expect(useAuthStore.getState().organizations).toEqual([])
      expect(useAuthStore.getState().currentOrg).toBeNull()
    })
  })

  describe('setCurrentOrg', () => {
    it('should set current organization', () => {
      const org = { id: 1, name: 'Test Org', slug: 'test-org', role: 'owner' }
      useAuthStore.getState().setCurrentOrg(org)
      expect(useAuthStore.getState().currentOrg).toEqual(org)
    })
  })

  describe('logout', () => {
    it('should clear all auth state', () => {
      useAuthStore.setState({
        user: { id: 1, email: 'test@example.com', username: 'testuser' },
        currentOrg: { id: 1, name: 'Org', slug: 'org', role: 'owner' },
        organizations: [{ id: 1, name: 'Org', slug: 'org', role: 'owner' }],
      })
      useAuthStore.getState().logout()
      const state = useAuthStore.getState()
      expect(state.user).toBeNull()
      expect(state.currentOrg).toBeNull()
      expect(state.organizations).toEqual([])
    })
  })

  describe('isAuthenticated', () => {
    it('should return false when no user', () => {
      expect(useAuthStore.getState().isAuthenticated()).toBe(false)
    })

    it('should return true when user exists', () => {
      useAuthStore.setState({ user: { id: 1, email: 'test@example.com', username: 'testuser' } })
      expect(useAuthStore.getState().isAuthenticated()).toBe(true)
    })

    it('should return false after logout', () => {
      useAuthStore.setState({ user: { id: 1, email: 'test@example.com', username: 'testuser' } })
      useAuthStore.getState().logout()
      expect(useAuthStore.getState().isAuthenticated()).toBe(false)
    })
  })
})
