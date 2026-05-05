import { describe, it, expect, beforeEach } from 'vitest'
import { useOrganizationStore, Organization, OrganizationMember } from '../organization'
import { getOrgState, parseWasmAny } from '@/lib/wasm-core'

const orgs = () => JSON.parse(getOrgState().organizations_json()) as Organization[]
const curOrg = () => parseWasmAny<Organization>(getOrgState().current_org_json())
const members = () => JSON.parse(getOrgState().members_json()) as OrganizationMember[]

describe('useOrganizationStore', () => {
  const mockOrg: Organization = {
    id: 1,
    name: 'Test Org',
    slug: 'test-org',
    logo_url: 'https://example.com/logo.png',
    subscription_plan: 'pro',
    subscription_status: 'active',
    created_at: '2024-01-01T00:00:00Z',
    updated_at: '2024-01-01T00:00:00Z',
  }

  const mockMember: OrganizationMember = {
    id: 1,
    user_id: 1,
    username: 'testuser',
    email: 'test@example.com',
    name: 'Test User',
    avatar_url: 'https://example.com/avatar.png',
    role: 'owner',
    joined_at: '2024-01-01T00:00:00Z',
  }

  beforeEach(() => {
    getOrgState().set_organizations('[]')
    getOrgState().set_current_org('')
    getOrgState().set_members('[]')
    useOrganizationStore.setState({ _tick: 0, isLoading: false, error: null })
  })

  describe('initial state', () => {
    it('should have empty organizations', () => {
      expect(orgs()).toEqual([])
    })

    it('should have null currentOrganization', () => {
      expect(curOrg()).toBeNull()
    })

    it('should have empty members', () => {
      expect(members()).toEqual([])
    })
  })

  describe('setOrganizations', () => {
    it('should set organizations', () => {
      useOrganizationStore.getState().setOrganizations([mockOrg])

      expect(orgs()).toHaveLength(1)
      expect(orgs()[0]).toEqual(mockOrg)
    })
  })

  describe('setCurrentOrganization', () => {
    it('should set current organization', () => {
      useOrganizationStore.getState().setCurrentOrganization(mockOrg)

      expect(curOrg()).toEqual(mockOrg)
    })

    it('should set current organization to null', () => {
      useOrganizationStore.getState().setCurrentOrganization(mockOrg)
      useOrganizationStore.getState().setCurrentOrganization(null)

      expect(curOrg()).toBeNull()
    })
  })

  describe('addOrganization', () => {
    it('should add organization', () => {
      useOrganizationStore.getState().addOrganization(mockOrg)

      expect(orgs()).toHaveLength(1)
    })

    it('should append to existing organizations', () => {
      const org2: Organization = { ...mockOrg, id: 2, slug: 'org-2' }
      useOrganizationStore.getState().setOrganizations([mockOrg])

      useOrganizationStore.getState().addOrganization(org2)

      expect(orgs()).toHaveLength(2)
    })
  })

  describe('updateOrganization', () => {
    it('should update organization in list', () => {
      useOrganizationStore.getState().setOrganizations([mockOrg])

      useOrganizationStore.getState().updateOrganization(1, { name: 'Updated Name' })

      expect(orgs()[0].name).toBe('Updated Name')
    })

    it('should update currentOrganization if same id', () => {
      useOrganizationStore.getState().setOrganizations([mockOrg])
      useOrganizationStore.getState().setCurrentOrganization(mockOrg)

      useOrganizationStore.getState().updateOrganization(1, { name: 'Updated Name' })

      expect(curOrg()?.name).toBe('Updated Name')
    })

    it('should not update currentOrganization if different id', () => {
      const org2: Organization = { ...mockOrg, id: 2 }
      useOrganizationStore.getState().setOrganizations([mockOrg, org2])
      useOrganizationStore.getState().setCurrentOrganization(mockOrg)

      useOrganizationStore.getState().updateOrganization(2, { name: 'Updated Name' })

      expect(curOrg()?.name).toBe('Test Org')
    })
  })

  describe('removeOrganization', () => {
    it('should remove organization from list', () => {
      useOrganizationStore.getState().setOrganizations([mockOrg])

      useOrganizationStore.getState().removeOrganization(1)

      expect(orgs()).toHaveLength(0)
    })

    it('should clear currentOrganization if removed', () => {
      useOrganizationStore.getState().setOrganizations([mockOrg])
      useOrganizationStore.getState().setCurrentOrganization(mockOrg)

      useOrganizationStore.getState().removeOrganization(1)

      expect(curOrg()).toBeNull()
    })

    it('should not clear currentOrganization if different id', () => {
      const org2: Organization = { ...mockOrg, id: 2 }
      useOrganizationStore.getState().setOrganizations([mockOrg, org2])
      useOrganizationStore.getState().setCurrentOrganization(mockOrg)

      useOrganizationStore.getState().removeOrganization(2)

      expect(curOrg()).toEqual(mockOrg)
    })
  })

  describe('member management', () => {
    describe('setMembers', () => {
      it('should set members', () => {
        useOrganizationStore.getState().setMembers([mockMember])

        expect(members()).toHaveLength(1)
        expect(members()[0]).toEqual(mockMember)
      })
    })

    describe('addMember', () => {
      it('should add member', () => {
        useOrganizationStore.getState().addMember(mockMember)

        expect(members()).toHaveLength(1)
      })
    })

    describe('updateMember', () => {
      it('should update member by user_id', () => {
        useOrganizationStore.getState().setMembers([mockMember])

        useOrganizationStore.getState().updateMember(1, { role: 'admin' })

        expect(members()[0].role).toBe('admin')
      })

      it('should not update non-matching member', () => {
        useOrganizationStore.getState().setMembers([mockMember])

        useOrganizationStore.getState().updateMember(999, { role: 'admin' })

        expect(members()[0].role).toBe('owner')
      })
    })

    describe('removeMember', () => {
      it('should remove member by user_id', () => {
        useOrganizationStore.getState().setMembers([mockMember])

        useOrganizationStore.getState().removeMember(1)

        expect(members()).toHaveLength(0)
      })
    })
  })

  describe('setLoading', () => {
    it('should set loading state', () => {
      useOrganizationStore.getState().setLoading(true)

      const state = useOrganizationStore.getState()
      expect(state.isLoading).toBe(true)
    })
  })

  describe('setError', () => {
    it('should set error', () => {
      useOrganizationStore.getState().setError('Test error')

      const state = useOrganizationStore.getState()
      expect(state.error).toBe('Test error')
    })
  })

  describe('reset', () => {
    it('should reset to initial state', () => {
      useOrganizationStore.getState().setOrganizations([mockOrg])
      useOrganizationStore.getState().setCurrentOrganization(mockOrg)
      useOrganizationStore.getState().setMembers([mockMember])
      useOrganizationStore.setState({ isLoading: true, error: 'Some error' })

      useOrganizationStore.getState().reset()

      expect(orgs()).toEqual([])
      expect(curOrg()).toBeNull()
      expect(members()).toEqual([])
      expect(useOrganizationStore.getState().isLoading).toBe(false)
      expect(useOrganizationStore.getState().error).toBeNull()
    })
  })
})
