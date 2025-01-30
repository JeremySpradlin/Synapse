/**
 * Synapse - Frontend Window Management
 * 
 * This module handles the frontend window management, animations,
 * and focus handling for the Synapse launcher.
 */

import { getCurrent, PhysicalPosition } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'

/** Message interface for chat messages */
interface ChatMessage {
  id: string
  content: string
  timestamp: Date
  type: 'sent' | 'received'
}

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
 * Creates a chat message element
 */
function createMessageElement(message: ChatMessage): HTMLDivElement {
  const messageEl = document.createElement('div')
  messageEl.className = `chat-message ${message.type}`
  
  const contentEl = document.createElement('div')
  contentEl.className = 'message-content'
  contentEl.textContent = message.content
  
  const timeEl = document.createElement('div')
  timeEl.className = 'message-time'
  timeEl.textContent = message.timestamp.toLocaleTimeString([], { 
    hour: '2-digit', 
    minute: '2-digit' 
  })
  
  messageEl.appendChild(contentEl)
  messageEl.appendChild(timeEl)
  
  return messageEl
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

  /** Chat messages array */
  private messages: ChatMessage[] = []
  
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
  /** Reference to the chat history element */
  private chatHistory: HTMLDivElement | null = null
  /** Reference to the settings button */
  private settingsButton: HTMLButtonElement | null = null

  /** Focus management state */
  private focusState = {
    /** The element that should receive focus when window is shown */
    defaultFocusTarget: null as HTMLElement | null,
    /** The last focused element before window was hidden */
    lastFocusedElement: null as HTMLElement | null,
  }

  constructor() {
    this.mainWindow = getCurrent()
    void this.initWindow()
  }

  /**
   * Initializes the window manager and sets up event listeners
   */
  private async initWindow() {
    try {
      // Initialize UI elements
      await this.initializeUIElements()
      
      // Setup all event handlers
      await this.setupEventListeners()
      this.setupFocusManagement()
      this.setupKeyboardTrapping()
      this.setupChatHandlers()

      // Set default focus target
      if (this.chatInput) {
        this.focusState.defaultFocusTarget = this.chatInput
      }
    } catch (err) {
      console.error('Failed to initialize window:', err)
    }
  }

  /**
   * Initializes UI element references
   */
  private async initializeUIElements() {
    this.chatInput = document.querySelector('.chat-input')
    this.sendButton = document.querySelector('.send-button')
    this.chatHistory = document.querySelector('.chat-history')
    this.settingsButton = document.querySelector('.settings-button')

    if (!this.chatInput || !this.sendButton || !this.chatHistory || !this.settingsButton) {
      console.error('Failed to initialize UI elements')
    }

    // Set up settings button click handler
    this.settingsButton?.addEventListener('click', async () => {
      try {
        await invoke('open_settings_window')
      } catch (err) {
        console.error('Failed to open settings:', err)
      }
    })
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
    if (!this.chatInput || !this.chatHistory) return

    const content = this.chatInput.value.trim()
    if (content) {
      // Create new message
      const message: ChatMessage = {
        id: crypto.randomUUID(),
        content,
        timestamp: new Date(),
        type: 'sent'
      }
      
      // Add message to state
      this.messages.push(message)
      
      // Create and append message element
      const messageEl = createMessageElement(message)
      this.chatHistory.appendChild(messageEl)
      
      // Clear input and scroll to bottom
      this.chatInput.value = ''
      this.scrollToBottom()
      
      // TODO: Handle message processing and response
      this.simulateResponse()
    }
  }

  /**
   * Temporary function to simulate a response
   * This will be replaced with actual message processing
   */
  private simulateResponse() {
    setTimeout(() => {
      const response: ChatMessage = {
        id: crypto.randomUUID(),
        content: 'This is a simulated response.',
        timestamp: new Date(),
        type: 'received'
      }
      
      this.messages.push(response)
      
      if (this.chatHistory) {
        const messageEl = createMessageElement(response)
        this.chatHistory.appendChild(messageEl)
        this.scrollToBottom()
      }
    }, 1000)
  }

  /**
   * Scrolls the chat history to the bottom
   */
  private scrollToBottom() {
    if (this.chatHistory) {
      this.chatHistory.scrollTop = this.chatHistory.scrollHeight
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
      // Window visibility events
      await this.mainWindow.listen('toggle_window', () => {
        void this.toggleWindowVisibility()
      })

      // Window state events
      await this.mainWindow.listen('window_shown', () => {
        void this.handleWindowShown()
      })

      await this.mainWindow.listen('window_hidden', () => {
        void this.handleWindowHidden()
      })
    } catch (err) {
      console.error('Failed to setup event listeners:', err)
    }
  }

  /**
   * Handles window visibility toggling
   */
  private async toggleWindowVisibility() {
    if (this.state.isVisible) {
      void this.hideWindow()
    } else {
      void this.showWindow()
    }
  }

  /**
   * Sets up focus management for the window
   */
  private setupFocusManagement() {
    // Store focused element when focus changes within the window
    document.addEventListener('focus', (e) => {
      if (e.target instanceof HTMLElement && this.state.isVisible) {
        this.focusState.lastFocusedElement = e.target
      }
    }, true)

    // Prevent focus from leaving the window when visible
    document.addEventListener('focusin', (e) => {
      if (!this.state.isVisible) return

      const target = e.target as HTMLElement
      if (!document.body.contains(target)) {
        this.restoreFocus()
      }
    })
  }

  /**
   * Handles window shown event
   */
  private async handleWindowShown() {
    this.state.isVisible = true
    this.restoreFocus()
  }

  /**
   * Handles window hidden event
   */
  private async handleWindowHidden() {
    this.state.isVisible = false
    this.cleanupBeforeHide()
  }

  /**
   * Restores focus to the appropriate element
   */
  private restoreFocus() {
    // Try to focus the last focused element
    if (this.focusState.lastFocusedElement?.isConnected) {
      this.focusState.lastFocusedElement.focus()
      return
    }

    // Fall back to default focus target
    if (this.focusState.defaultFocusTarget?.isConnected) {
      this.focusState.defaultFocusTarget.focus()
      return
    }

    // Last resort: focus first focusable element
    const focusableElements = this.getFocusableElements()
    if (focusableElements.length > 0) {
      focusableElements[0].focus()
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
      
      this.state.initialY = -600
      this.state.targetY = 0
      this.state.currentY = this.state.initialY
      
      void this.animateWindow(centerX)
      await this.mainWindow.emit('window_shown', null)
    } catch (err) {
      console.error('Failed to show window:', err)
    }
  }

  /**
   * Hides the window with an animation
   */
  private async hideWindow() {
    this.cancelCurrentAnimation()
    
    try {
      this.state.initialY = this.state.currentY
      this.state.targetY = -600
      
      const currentPosition = await this.mainWindow.outerPosition()
      void this.animateWindow(currentPosition.x, true)
      await this.mainWindow.emit('window_hidden', null)
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