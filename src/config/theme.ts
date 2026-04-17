import type { GlobalThemeOverrides } from 'naive-ui'

export const appThemeOverrides: GlobalThemeOverrides = {
  common: {
    baseColor: '#111418',
    fontFamily: '"Smiley Sans", "Microsoft YaHei", "PingFang SC", sans-serif',
    fontFamilyMono: '"Cascadia Mono", "JetBrains Mono", "SFMono-Regular", monospace',
    bodyColor: 'rgba(17, 20, 24, 0.72)',
    cardColor: 'rgba(255, 255, 255, 0.06)',
    modalColor: 'rgba(24, 28, 34, 0.96)',
    popoverColor: 'rgba(24, 28, 34, 0.96)',
    tableColor: 'rgba(255, 255, 255, 0.04)',
    actionColor: 'rgba(255, 255, 255, 0.06)',
    hoverColor: 'rgba(255, 255, 255, 0.1)',
    pressedColor: 'rgba(255, 255, 255, 0.14)',
    borderColor: 'rgba(255, 255, 255, 0.1)',
    dividerColor: 'rgba(255, 255, 255, 0.08)',
    textColorBase: '#E6EDF3',
    textColor1: '#F5F7FA',
    textColor2: 'rgba(230, 237, 243, 0.82)',
    textColor3: 'rgba(230, 237, 243, 0.52)',
    placeholderColor: 'rgba(230, 237, 243, 0.38)',
    inputColor: 'rgba(255, 255, 255, 0.06)',
    inputColorDisabled: 'rgba(255, 255, 255, 0.04)',
    primaryColor: 'rgba(255, 255, 255, 0.9)',
    primaryColorHover: '#FFFFFF',
    primaryColorPressed: 'rgba(255, 255, 255, 0.72)',
    primaryColorSuppl: 'rgba(255, 255, 255, 0.82)',
    infoColor: 'rgba(255, 255, 255, 0.9)',
  },
  Switch: {
    railColorActive: 'rgba(255, 255, 255, 0.8)',
  },
  Card: {
    borderRadius: '10px',
  },
  Button: {
    textColor: '#F5F7FA',
  },
}
