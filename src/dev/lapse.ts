import '@aiforui/lapse/install'
import { mountLapse } from '@aiforui/lapse/panel'

const panel = mountLapse({ position: 'bottom-left' })

if (import.meta.hot) {
  import.meta.hot.dispose(() => panel.unmount())
}
