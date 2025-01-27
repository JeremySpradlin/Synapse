// src/main.ts
import { Window, PhysicalPosition } from '@tauri-apps/api/window'

interface WindowState {
  isVisible: boolean
  initialY: number
  targetY: number
  currentY: number
  animationFrame: number | null
}

class WindowManager {
  private state: WindowState = {
    isVisible: false,
    initialY: -600, // Initial position above screen
    targetY: 0,     // Target position when visible
    currentY: -600,
    animationFrame: null
  }

  private readonly ANIMATION_DURATION = 300 // ms
  private readonly EASING = (t: number) => t * (2 - t) // Ease out quad
  private mainWindow: Window

  constructor() {
    this.mainWindow = new Window('main')
    this.initWindow()
  }

  private async initWindow() {
    try {
      await this.setupEventListeners()
    } catch (err) {
      console.error('Failed to initialize window:', err)
    }
  }

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

  private async showWindow() {
    // Cancel any ongoing animation
    if (this.state.animationFrame !== null) {
      cancelAnimationFrame(this.state.animationFrame)
    }

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
      
      const startTime = performance.now()
      
      const animate = async (currentTime: number) => {
        const elapsed = currentTime - startTime
        const progress = Math.min(elapsed / this.ANIMATION_DURATION, 1)
        const easedProgress = this.EASING(progress)
        
        this.state.currentY = this.state.initialY + (this.state.targetY - this.state.initialY) * easedProgress
        
        try {
          await this.mainWindow.setPosition(new PhysicalPosition(Math.round(centerX), Math.round(this.state.currentY)))
        } catch (e) {
          console.error('Failed to set window position:', e)
        }
        
        if (progress < 1) {
          this.state.animationFrame = requestAnimationFrame(animate)
        }
      }
      
      this.state.animationFrame = requestAnimationFrame(animate)
    } catch (err) {
      console.error('Failed to show window:', err)
    }
  }

  private async hideWindow() {
    if (this.state.animationFrame !== null) {
      cancelAnimationFrame(this.state.animationFrame)
    }
    
    try {
      const startTime = performance.now()
      
      this.state.isVisible = false
      this.state.initialY = this.state.currentY
      this.state.targetY = -600
      
      const animate = async (currentTime: number) => {
        const elapsed = currentTime - startTime
        const progress = Math.min(elapsed / this.ANIMATION_DURATION, 1)
        const easedProgress = this.EASING(progress)
        
        this.state.currentY = this.state.initialY + (this.state.targetY - this.state.initialY) * easedProgress
        
        try {
          const currentPosition = await this.mainWindow.outerPosition()
          await this.mainWindow.setPosition(new PhysicalPosition(currentPosition.x, Math.round(this.state.currentY)))
        } catch (e) {
          console.error('Failed to set window position:', e)
        }
        
        if (progress < 1) {
          this.state.animationFrame = requestAnimationFrame(animate)
        } else {
          await this.mainWindow.hide()
        }
      }
      
      this.state.animationFrame = requestAnimationFrame(animate)
    } catch (err) {
      console.error('Failed to hide window:', err)
    }
  }
}

// Initialize the window manager
new WindowManager()