import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { PromoCodeInput } from './PromoCodeInput'
import { getPromoCodeService } from '@/lib/wasm-core'

const mockT = (key: string) => {
  const translations: Record<string, string> = {
    placeholder: 'Enter promo code',
    validate: 'Validate',
    validating: 'Validating...',
    redeem: 'Redeem',
    redeeming: 'Redeeming...',
    enterCode: 'Please enter a promo code',
    invalid: 'Invalid promo code',
    validateError: 'Failed to validate promo code',
    redeemError: 'Failed to redeem promo code',
    valid: 'Valid promo code',
    plan: 'Plan',
    duration: 'Duration',
    months: 'months',
    confirmRedeem: 'Click redeem to activate',
    'errors.promo_code_not_found': 'Promo code not found',
    'errors.promo_code_not_owner': 'Only owner can redeem',
  }
  return translations[key] || key
}

const mockValidate = vi.fn().mockResolvedValue('{}')
const mockRedeem = vi.fn().mockResolvedValue('{}')

vi.mocked(getPromoCodeService).mockReturnValue({
  validate: mockValidate,
  redeem: mockRedeem,
  get_history: vi.fn().mockResolvedValue('{"codes":[]}'),
} as unknown as ReturnType<typeof getPromoCodeService>)

describe('PromoCodeInput', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(getPromoCodeService).mockReturnValue({
      validate: mockValidate,
      redeem: mockRedeem,
      get_history: vi.fn().mockResolvedValue('{"codes":[]}'),
    } as unknown as ReturnType<typeof getPromoCodeService>)
  })

  it('renders input and validate button', () => {
    render(<PromoCodeInput t={mockT} />)
    expect(screen.getByPlaceholderText('Enter promo code')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Validate' })).toBeInTheDocument()
  })

  it('converts input to uppercase', async () => {
    render(<PromoCodeInput t={mockT} />)
    const input = screen.getByPlaceholderText('Enter promo code')
    await userEvent.type(input, 'test123')
    expect(input).toHaveValue('TEST123')
  })

  it('disables validate button when code is empty', () => {
    render(<PromoCodeInput t={mockT} />)
    const button = screen.getByRole('button', { name: 'Validate' })
    expect(button).toBeDisabled()
  })

  it('validates promo code successfully', async () => {
    mockValidate.mockResolvedValue(JSON.stringify({
      valid: true, code: 'TEST123', plan_name: 'pro',
      plan_display_name: 'Pro', duration_months: 3,
    }))

    const onValidate = vi.fn()
    render(<PromoCodeInput onValidate={onValidate} t={mockT} />)

    const input = screen.getByPlaceholderText('Enter promo code')
    await userEvent.type(input, 'TEST123')
    await userEvent.click(screen.getByRole('button', { name: 'Validate' }))

    await waitFor(() => {
      expect(screen.getByText('Valid promo code')).toBeInTheDocument()
    })
    expect(screen.getByText(/Plan: Pro/)).toBeInTheDocument()
    expect(screen.getByText(/Duration: 3 months/)).toBeInTheDocument()
    expect(onValidate).toHaveBeenCalledWith({
      valid: true, code: 'TEST123', plan_name: 'pro',
      plan_display_name: 'Pro', duration_months: 3,
    })
  })

  it('shows error for invalid promo code', async () => {
    mockValidate.mockResolvedValue(JSON.stringify({
      valid: false, code: 'INVALID', message_code: 'promo_code_not_found',
    }))

    render(<PromoCodeInput t={mockT} />)
    const input = screen.getByPlaceholderText('Enter promo code')
    await userEvent.type(input, 'INVALID')
    await userEvent.click(screen.getByRole('button', { name: 'Validate' }))

    await waitFor(() => {
      expect(screen.getByText('Promo code not found')).toBeInTheDocument()
    })
  })

  it('shows error on validate API failure', async () => {
    mockValidate.mockRejectedValue(new Error('Network error'))

    render(<PromoCodeInput t={mockT} />)
    const input = screen.getByPlaceholderText('Enter promo code')
    await userEvent.type(input, 'TEST123')
    await userEvent.click(screen.getByRole('button', { name: 'Validate' }))

    await waitFor(() => {
      expect(screen.getByText('Failed to validate promo code')).toBeInTheDocument()
    })
  })

  it('redeems promo code after validation', async () => {
    mockValidate.mockResolvedValue(JSON.stringify({
      valid: true, code: 'TEST123', plan_name: 'pro',
      plan_display_name: 'Pro', duration_months: 3,
    }))
    mockRedeem.mockResolvedValue('{}')

    const onRedeemSuccess = vi.fn()
    render(<PromoCodeInput onRedeemSuccess={onRedeemSuccess} t={mockT} />)

    const input = screen.getByPlaceholderText('Enter promo code')
    await userEvent.type(input, 'TEST123')
    await userEvent.click(screen.getByRole('button', { name: 'Validate' }))

    await waitFor(() => {
      expect(screen.getByText('Valid promo code')).toBeInTheDocument()
    })

    await userEvent.click(screen.getByRole('button', { name: 'Redeem' }))

    await waitFor(() => {
      expect(onRedeemSuccess).toHaveBeenCalledWith({ success: true })
    })
    expect(input).toHaveValue('')
  })

  it('shows error on redeem failure', async () => {
    mockValidate.mockResolvedValue(JSON.stringify({
      valid: true, code: 'TEST123', plan_name: 'pro',
      plan_display_name: 'Pro', duration_months: 3,
    }))
    mockRedeem.mockRejectedValue(new Error('Redeem failed'))

    render(<PromoCodeInput t={mockT} />)
    const input = screen.getByPlaceholderText('Enter promo code')
    await userEvent.type(input, 'TEST123')
    await userEvent.click(screen.getByRole('button', { name: 'Validate' }))

    await waitFor(() => {
      expect(screen.getByText('Valid promo code')).toBeInTheDocument()
    })

    await userEvent.click(screen.getByRole('button', { name: 'Redeem' }))

    await waitFor(() => {
      expect(screen.getByText('Failed to redeem promo code')).toBeInTheDocument()
    })
  })

  it('disables input and button when disabled prop is true', () => {
    render(<PromoCodeInput disabled={true} t={mockT} />)
    expect(screen.getByPlaceholderText('Enter promo code')).toBeDisabled()
    expect(screen.getByRole('button', { name: 'Validate' })).toBeDisabled()
  })

  it('handles Enter key to validate', async () => {
    mockValidate.mockResolvedValue(JSON.stringify({
      valid: true, code: 'TEST123', plan_name: 'pro',
      plan_display_name: 'Pro', duration_months: 3,
    }))

    render(<PromoCodeInput t={mockT} />)
    const input = screen.getByPlaceholderText('Enter promo code')
    await userEvent.type(input, 'TEST123')
    await userEvent.keyboard('{Enter}')

    await waitFor(() => {
      expect(mockValidate).toHaveBeenCalledWith(JSON.stringify({ code: 'TEST123' }))
    })
  })

  it('clears validation when code changes', async () => {
    mockValidate.mockResolvedValue(JSON.stringify({
      valid: true, code: 'TEST123', plan_name: 'pro',
      plan_display_name: 'Pro', duration_months: 3,
    }))

    render(<PromoCodeInput t={mockT} />)
    const input = screen.getByPlaceholderText('Enter promo code')
    await userEvent.type(input, 'TEST123')
    await userEvent.click(screen.getByRole('button', { name: 'Validate' }))

    await waitFor(() => {
      expect(screen.getByText('Valid promo code')).toBeInTheDocument()
    })

    await userEvent.clear(input)
    await userEvent.type(input, 'NEW')
    expect(screen.queryByText('Valid promo code')).not.toBeInTheDocument()
  })
})
