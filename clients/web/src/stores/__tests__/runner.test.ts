import { describe, it, expect, beforeEach, vi } from 'vitest'
import { getRunnerStatusInfo, canAcceptPods, formatHostInfo, Runner, useRunnerStore } from '../runner'

let mockRunnersList: Runner[] = []
let mockAvailableRunners: Runner[] = []
let mockCurrentRunner: Runner | null = null

const mockService = {
  fetch_runners: vi.fn(),
  fetch_available_runners: vi.fn(),
  fetch_runner: vi.fn(),
  update_runner: vi.fn(),
  delete_runner: vi.fn(),
  create_token: vi.fn(),
  fetch_tokens: vi.fn(),
  delete_token: vi.fn(),
  set_current_runner: vi.fn(),
  update_runner_status: vi.fn(),
  remove_runner_local: vi.fn(),
  runners_json: vi.fn(() => JSON.stringify(mockRunnersList)),
  available_runners_json: vi.fn(() => JSON.stringify(mockAvailableRunners)),
  current_runner_json: vi.fn(() => (mockCurrentRunner ? JSON.stringify(mockCurrentRunner) : null)),
}

vi.mock('@/lib/wasm-core', () => ({
  getRunnerService: () => mockService,
}))

const createMockRunner = (overrides: Partial<Runner> = {}): Runner => ({
  id: 1,
  node_id: 'test-runner',
  status: 'online',
  is_enabled: true,
  current_pods: 0,
  max_concurrent_pods: 5,
  last_heartbeat: '2024-01-01T00:00:00Z',
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-01T00:00:00Z',
  ...overrides,
})

const getRunners = (): Runner[] => JSON.parse(mockService.runners_json())
const getAvailableRunners = (): Runner[] => JSON.parse(mockService.available_runners_json())
const getCurrentRunner = (): Runner | null => {
  const raw = mockService.current_runner_json()
  return raw ? JSON.parse(raw) : null
}

beforeEach(() => {
  mockRunnersList = []
  mockAvailableRunners = []
  mockCurrentRunner = null
  useRunnerStore.setState({
    _tick: 0,
    loading: false,
    error: null,
  })
  vi.clearAllMocks()
})

describe('Runner Store Actions', () => {
  describe('fetchRunners', () => {
    it('should fetch runners successfully', async () => {
      const runners = [createMockRunner({ id: 1 }), createMockRunner({ id: 2, node_id: 'runner-2' })]
      mockService.fetch_runners.mockImplementation(async () => {
        mockRunnersList = runners
        return JSON.stringify({ runners })
      })

      await useRunnerStore.getState().fetchRunners()

      expect(mockService.fetch_runners).toHaveBeenCalled()
      expect(getRunners()).toEqual(runners)
      expect(useRunnerStore.getState().loading).toBe(false)
    })

    it('should filter runners by status', async () => {
      mockService.fetch_runners.mockResolvedValue(JSON.stringify({ runners: [] }))

      await useRunnerStore.getState().fetchRunners('online')

      expect(mockService.fetch_runners).toHaveBeenCalledWith('online')
    })

    it('should handle fetch error', async () => {
      mockService.fetch_runners.mockRejectedValue(new Error('Network error'))

      await useRunnerStore.getState().fetchRunners()

      expect(useRunnerStore.getState().error).toBe('Network error')
      expect(useRunnerStore.getState().loading).toBe(false)
    })
  })

  describe('fetchAvailableRunners', () => {
    it('should fetch available runners successfully', async () => {
      const runners = [createMockRunner()]
      mockService.fetch_available_runners.mockImplementation(async () => {
        mockAvailableRunners = runners
        return JSON.stringify({ runners })
      })

      await useRunnerStore.getState().fetchAvailableRunners()

      expect(getAvailableRunners()).toEqual(runners)
    })

    it('should handle fetch error', async () => {
      mockService.fetch_available_runners.mockRejectedValue(new Error('Network error'))

      await useRunnerStore.getState().fetchAvailableRunners()

      expect(useRunnerStore.getState().error).toBe('Network error')
    })
  })

  describe('fetchRunner', () => {
    it('should fetch single runner successfully', async () => {
      const runner = createMockRunner()
      mockService.fetch_runner.mockImplementation(async () => {
        mockCurrentRunner = runner
        return JSON.stringify(runner)
      })

      await useRunnerStore.getState().fetchRunner(1)

      expect(mockService.fetch_runner).toHaveBeenCalledWith(BigInt(1))
      expect(getCurrentRunner()).toEqual(runner)
    })

    it('should handle fetch error', async () => {
      mockService.fetch_runner.mockRejectedValue(new Error('Not found'))

      await useRunnerStore.getState().fetchRunner(999)

      expect(useRunnerStore.getState().error).toBe('Not found')
    })
  })

  describe('updateRunner', () => {
    it('should update runner successfully', async () => {
      const existingRunner = createMockRunner()
      const updatedRunner = { ...existingRunner, description: 'Updated description' }

      mockService.fetch_runners.mockImplementation(async () => {
        mockRunnersList = [existingRunner]
        return JSON.stringify({ runners: [existingRunner] })
      })
      await useRunnerStore.getState().fetchRunners()

      mockService.update_runner.mockImplementation(async () => {
        mockRunnersList = [updatedRunner]
        return JSON.stringify(updatedRunner)
      })

      const result = await useRunnerStore.getState().updateRunner(1, { description: 'Updated description' })

      expect(result).toEqual(updatedRunner)
      expect(getRunners()[0].description).toBe('Updated description')
    })

    it('should handle update error', async () => {
      mockService.update_runner.mockRejectedValue(new Error('Update failed'))

      await expect(useRunnerStore.getState().updateRunner(1, { description: 'test' })).rejects.toThrow()
      expect(useRunnerStore.getState().error).toBe('Update failed')
    })
  })

  describe('deleteRunner', () => {
    it('should delete runner successfully', async () => {
      const runner = createMockRunner()
      mockService.fetch_runners.mockImplementation(async () => {
        mockRunnersList = [runner]
        return JSON.stringify({ runners: [runner] })
      })
      await useRunnerStore.getState().fetchRunners()
      mockService.fetch_available_runners.mockImplementation(async () => {
        mockAvailableRunners = [runner]
        return JSON.stringify({ runners: [runner] })
      })
      await useRunnerStore.getState().fetchAvailableRunners()

      mockService.delete_runner.mockImplementation(async () => {
        mockRunnersList = []
        mockAvailableRunners = []
      })

      await useRunnerStore.getState().deleteRunner(1)

      expect(getRunners()).toHaveLength(0)
      expect(getAvailableRunners()).toHaveLength(0)
    })

    it('should handle delete error', async () => {
      mockService.delete_runner.mockRejectedValue(new Error('Delete failed'))

      await expect(useRunnerStore.getState().deleteRunner(1)).rejects.toThrow()
    })
  })

  describe('createToken', () => {
    it('should create token successfully', async () => {
      mockService.create_token.mockResolvedValue(
        JSON.stringify({ token: 'new-token-123', expires_at: '2024-12-31T23:59:59Z', message: 'Token created' }),
      )

      const token = await useRunnerStore.getState().createToken()

      expect(token).toBe('new-token-123')
      expect(mockService.create_token).toHaveBeenCalled()
    })

    it('should handle create token error', async () => {
      mockService.create_token.mockRejectedValue(new Error('Failed to create token'))

      await expect(useRunnerStore.getState().createToken()).rejects.toThrow()
      expect(useRunnerStore.getState().error).toBe('Failed to create token')
    })
  })

  describe('setCurrentRunner and updateRunnerStatus', () => {
    it('should set current runner', () => {
      const runner = createMockRunner()
      mockService.set_current_runner.mockImplementation(() => {
        mockCurrentRunner = runner
      })

      useRunnerStore.getState().setCurrentRunner(runner)

      expect(getCurrentRunner()).toEqual(runner)
    })

    it('should clear current runner', () => {
      mockCurrentRunner = createMockRunner()
      mockService.set_current_runner.mockImplementation(() => {
        mockCurrentRunner = null
      })

      useRunnerStore.getState().setCurrentRunner(null)

      expect(getCurrentRunner()).toBeNull()
    })

    it('should update runner status to offline', async () => {
      const runner = createMockRunner({ status: 'online' })
      mockService.fetch_runners.mockImplementation(async () => {
        mockRunnersList = [runner]
        return JSON.stringify({ runners: [runner] })
      })
      await useRunnerStore.getState().fetchRunners()
      mockService.fetch_available_runners.mockImplementation(async () => {
        mockAvailableRunners = [runner]
        return JSON.stringify({ runners: [runner] })
      })
      await useRunnerStore.getState().fetchAvailableRunners()

      mockService.update_runner_status.mockImplementation(() => {
        mockRunnersList = [{ ...runner, status: 'offline' }]
        mockAvailableRunners = []
      })

      useRunnerStore.getState().updateRunnerStatus(1, 'offline')

      expect(getRunners()[0].status).toBe('offline')
      expect(getAvailableRunners()).toHaveLength(0)
    })

    it('should keep available runners when status is online', async () => {
      const runner = createMockRunner()
      mockService.fetch_runners.mockImplementation(async () => {
        mockRunnersList = [runner]
        return JSON.stringify({ runners: [runner] })
      })
      await useRunnerStore.getState().fetchRunners()
      mockService.fetch_available_runners.mockImplementation(async () => {
        mockAvailableRunners = [runner]
        return JSON.stringify({ runners: [runner] })
      })
      await useRunnerStore.getState().fetchAvailableRunners()

      mockService.update_runner_status.mockImplementation(() => {
        // State stays the same since status is already online
      })

      useRunnerStore.getState().updateRunnerStatus(1, 'online')

      expect(getAvailableRunners()).toHaveLength(1)
    })
  })

  describe('clearError', () => {
    it('should clear error', () => {
      useRunnerStore.setState({ error: 'Some error' })
      useRunnerStore.getState().clearError()

      expect(useRunnerStore.getState().error).toBeNull()
    })
  })
})

describe('Runner Store Helper Functions', () => {
  describe('getRunnerStatusInfo', () => {
    it('should return correct info for online status', () => {
      const info = getRunnerStatusInfo('online')
      expect(info).toEqual({
        label: 'Online',
        color: 'text-green-600 dark:text-green-400',
        dotColor: 'bg-green-500',
      })
    })

    it('should return correct info for offline status', () => {
      const info = getRunnerStatusInfo('offline')
      expect(info).toEqual({
        label: 'Offline',
        color: 'text-gray-500 dark:text-gray-400',
        dotColor: 'bg-gray-400',
      })
    })

    it('should return correct info for maintenance status', () => {
      const info = getRunnerStatusInfo('maintenance')
      expect(info).toEqual({
        label: 'Maintenance',
        color: 'text-yellow-600 dark:text-yellow-400',
        dotColor: 'bg-yellow-500',
      })
    })

    it('should return correct info for busy status', () => {
      const info = getRunnerStatusInfo('busy')
      expect(info).toEqual({
        label: 'Busy',
        color: 'text-orange-600 dark:text-orange-400',
        dotColor: 'bg-orange-500',
      })
    })
  })

  describe('canAcceptPods', () => {
    const createRunner = (overrides: Partial<Runner> = {}): Runner => ({
      id: 1,
      node_id: 'test-runner',
      status: 'online',
      is_enabled: true,
      current_pods: 0,
      max_concurrent_pods: 5,
      last_heartbeat: '2024-01-01T00:00:00Z',
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z',
      ...overrides,
    })

    it('should return true for online runner with available slots', () => {
      const runner = createRunner({ status: 'online', current_pods: 2, max_concurrent_pods: 5 })
      expect(canAcceptPods(runner)).toBe(true)
    })

    it('should return false for offline runner', () => {
      const runner = createRunner({ status: 'offline' })
      expect(canAcceptPods(runner)).toBe(false)
    })

    it('should return false for maintenance runner', () => {
      const runner = createRunner({ status: 'maintenance' })
      expect(canAcceptPods(runner)).toBe(false)
    })

    it('should return false for busy runner', () => {
      const runner = createRunner({ status: 'busy' })
      expect(canAcceptPods(runner)).toBe(false)
    })

    it('should return false when at max capacity', () => {
      const runner = createRunner({ status: 'online', current_pods: 5, max_concurrent_pods: 5 })
      expect(canAcceptPods(runner)).toBe(false)
    })

    it('should return false when over max capacity', () => {
      const runner = createRunner({ status: 'online', current_pods: 6, max_concurrent_pods: 5 })
      expect(canAcceptPods(runner)).toBe(false)
    })

    it('should return true with zero current pods', () => {
      const runner = createRunner({ status: 'online', current_pods: 0, max_concurrent_pods: 5 })
      expect(canAcceptPods(runner)).toBe(true)
    })
  })

  describe('formatHostInfo', () => {
    it('should return "Unknown" for undefined host_info', () => {
      expect(formatHostInfo(undefined)).toBe('Unknown')
    })

    it('should return "Unknown" for empty host_info', () => {
      expect(formatHostInfo({})).toBe('Unknown')
    })

    it('should format os only', () => {
      const result = formatHostInfo({ os: 'linux' })
      expect(result).toBe('linux')
    })

    it('should format arch only', () => {
      const result = formatHostInfo({ arch: 'amd64' })
      expect(result).toBe('amd64')
    })

    it('should format cpu_cores only', () => {
      const result = formatHostInfo({ cpu_cores: 8 })
      expect(result).toBe('8 cores')
    })

    it('should format memory only', () => {
      const result = formatHostInfo({ memory: 17179869184 })
      expect(result).toBe('16.0GB RAM')
    })

    it('should format all fields', () => {
      const result = formatHostInfo({
        os: 'linux',
        arch: 'amd64',
        cpu_cores: 8,
        memory: 17179869184,
      })
      expect(result).toBe('linux / amd64 / 8 cores / 16.0GB RAM')
    })

    it('should format partial fields', () => {
      const result = formatHostInfo({
        os: 'darwin',
        cpu_cores: 4,
      })
      expect(result).toBe('darwin / 4 cores')
    })

    it('should handle small memory values', () => {
      const result = formatHostInfo({ memory: 1073741824 })
      expect(result).toBe('1.0GB RAM')
    })

    it('should handle large memory values', () => {
      const result = formatHostInfo({ memory: 137438953472 })
      expect(result).toBe('128.0GB RAM')
    })
  })
})
