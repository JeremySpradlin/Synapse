/**
 * Synapse - Frontend Window Management
 * 
 * This module handles the frontend window management, animations,
 * and focus handling for the Synapse launcher.
 */

import { getCurrent, PhysicalPosition } from '@tauri-apps/api/window'

/** State interface for window position and animation */
interface WindowState {
  /** Whether the window is currently visible */
  isVisible: boolean
  /** Initial Y position for animations */
  initialY: number
  /** Target Y position for animations */
  targetY: number
  /** Current Y position during animation */
  currentY: number
  /** Current animation frame ID */
  animationFrame: number | null
}

/**
 * WindowManager class handles all window-related operations including
 * animations, positioning, and focus management.
 */
class WindowManager {
  /** Window state for position tracking and animations */
  private state: WindowState = {
    isVisible: false,
    initialY: -600, // Initial position above screen
    targetY: 0,     // Target position when visible
    currentY: -600,
    animationFrame: null
  }

  /** Duration of show/hide animations in milliseconds */
  private readonly ANIMATION_DURATION = 300
  /** Easing function for smooth animations */
  private readonly EASING = (t: number) => t * (2 - t) // Ease out quad
  /** Reference to the main window */
  private mainWindow: Awaited<ReturnType<typeof getCurrent>>
  /** Stores the last focused element for focus restoration */
  private lastFocusedElement: HTMLElement | null = null
  /** Reference to the chat input element */
  private chatInput: HTMLInputElement | null = null
  /** Reference to the send button */
  private sendButton: HTMLButtonElement | null = null

  constructor() {
    this.mainWindow = getCurrent()
    this.initWindow()
  }

  /**
   * Initializes the window manager and sets up event listeners
   */
  private async initWindow() {
    try {
      // Initialize chat elements
      this.chatInput = document.querySelector('.chat-input')
      this.sendButton = document.querySelector('.send-button')
      
      // Setup event handlers
      await this.setupEventListeners()
      this.setupKeyboardTrapping()
      this.setupFocusManagement()
      this.setupChatHandlers()
    } catch (err) {
      console.error('Failed to initialize window:', err)
    }
  }

  /**
   * Sets up chat-specific event handlers
   */
  private setupChatHandlers() {
    if (!this.chatInput || !this.sendButton) return

    // Handle send button click
    this.sendButton.addEventListener('click', () => {
      this.handleSendMessage()
    })

    // Handle enter key in input
    this.chatInput.addEventListener('keydown', (e) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault()
        this.handleSendMessage()
      }
    })
  }

  /**
   * Handles sending a message
   */
  private handleSendMessage() {
    if (!this.chatInput) return

    const message = this.chatInput.value.trim()
    if (message) {
      // TODO: Implement message sending logic
      console.log('Sending message:', message)
      this.chatInput.value = ''
    }
  }

  /**
   * Sets up keyboard event handling for focus trapping
   */
  private setupKeyboardTrapping() {
    document.addEventListener('keydown', (e) => {
      if (!this.state.isVisible) return

      if (e.key === 'Tab') {
        this.handleTabKey(e)
      } else if (e.key === 'Escape') {
        void this.hideWindow()
      }
    })
  }

  /**
   * Sets up focus management for the window
   */
  private setupFocusManagement() {
    // Store last focused element before window shows
    document.addEventListener('focus', (e) => {
      if (e.target instanceof HTMLElement) {
        this.lastFocusedElement = e.target
      }
    }, true)

    // Prevent focus from leaving the window when visible
    document.addEventListener('focusin', (e) => {
      if (!this.state.isVisible) return

      const target = e.target as HTMLElement
      if (!document.body.contains(target)) {
        this.lastFocusedElement?.focus()
      }
    })
  }

  /**
   * Handles tab key navigation within the window
   */
  private handleTabKey(e: KeyboardEvent) {
    const focusableElements = this.getFocusableElements()
    if (focusableElements.length === 0) return

    const firstElement = focusableElements[0]
    const lastElement = focusableElements[focusableElements.length - 1]
    const activeElement = document.activeElement

    if (e.shiftKey && activeElement === firstElement) {
      e.preventDefault()
      lastElement.focus()
    } else if (!e.shiftKey && activeElement === lastElement) {
      e.preventDefault()
      firstElement.focus()
    }
  }

  /**
   * Gets all focusable elements within the window
   */
  private getFocusableElements(): HTMLElement[] {
    return Array.from(
      document.querySelectorAll<HTMLElement>(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      )
    ).filter(el => {
      const style = window.getComputedStyle(el)
      return style.display !== 'none' && style.visibility !== 'hidden'
    })
  }

  /**
   * Sets up window event listeners
   */
  private async setupEventListeners() {
    try {
      await this.mainWindow.listen('toggle_window', () => {
        if (this.state.isVisible) {
          void this.hideWindow()
        } else {
          void this.showWindow()
        }
      })
    } catch (err) {
      console.error('Failed to setup event listeners:', err)
    }
  }

  /**
   * Shows the window with an animation
   */
  private async showWindow() {
    this.cancelCurrentAnimation()

    try {
      await this.mainWindow.show()
      
      const size = await this.mainWindow.innerSize()
      const screenWidth = window.innerWidth
      
      // Center window horizontally
      const centerX = (screenWidth - size.width) / 2
      
      this.state.isVisible = true
      this.state.initialY = -600
      this.state.targetY = 0
      this.state.currentY = this.state.initialY
      
      // Focus the chat input
      this.focusChatInput()
      
      void this.animateWindow(centerX)
    } catch (err) {
      console.error('Failed to show window:', err)
    }
  }

  /**
   * Focuses the chat input
   */
  private focusChatInput() {
    setTimeout(() => {
      if (this.chatInput) {
        this.chatInput.focus()
      }
    }, 100)
  }

  /**
   * Hides the window with an animation
   */
  private async hideWindow() {
    this.cancelCurrentAnimation()
    
    try {
      this.state.isVisible = false
      this.state.initialY = this.state.currentY
      this.state.targetY = -600
      
      this.cleanupBeforeHide()
      
      const currentPosition = await this.mainWindow.outerPosition()
      void this.animateWindow(currentPosition.x, true)
    } catch (err) {
      console.error('Failed to hide window:', err)
    }
  }

  /**
   * Cleans up window state before hiding
   */
  private cleanupBeforeHide() {
    // Clear any active text selection
    window.getSelection()?.removeAllRanges()
    
    // Clear focus
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur()
    }

    // Clear input
    if (this.chatInput) {
      this.chatInput.value = ''
    }
  }

  /**
   * Cancels any ongoing animation
   */
  private cancelCurrentAnimation() {
    if (this.state.animationFrame !== null) {
      cancelAnimationFrame(this.state.animationFrame)
      this.state.animationFrame = null
    }
  }

  /**
   * Animates the window position
   */
  private async animateWindow(targetX: number, hideAfterAnimation = false) {
    const startTime = performance.now()
    
    const animate = async (currentTime: number) => {
      const elapsed = currentTime - startTime
      const progress = Math.min(elapsed / this.ANIMATION_DURATION, 1)
      const easedProgress = this.EASING(progress)
      
      this.state.currentY = this.state.initialY + 
        (this.state.targetY - this.state.initialY) * easedProgress
      
      try {
        await this.mainWindow.setPosition(
          new PhysicalPosition(
            Math.round(targetX),
            Math.round(this.state.currentY)
          )
        )
      } catch (e) {
        console.error('Failed to set window position:', e)
      }
      
      if (progress < 1) {
        this.state.animationFrame = requestAnimationFrame(animate)
      } else if (hideAfterAnimation) {
        await this.mainWindow.hide()
      }
    }
    
    this.state.animationFrame = requestAnimationFrame(animate)
  }
}

// Initialize the window manager
new WindowManager()