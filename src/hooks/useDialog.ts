import {message} from '@tauri-apps/api/dialog'

export default function useDialog() {
  const showErrorDialog = (msg: string) => {
    console.log(msg)  //TODO:dialog
  }

  const showSuccessDialog = (msg: string) => {
    console.log(msg)
  }

  const showInfoDialog = (msg: string) => {
    console.log(msg)
  }

  const showWarningDialog = async (msg: string) => {
    await message(msg, {
        type: 'warning',
        title: '提醒！',
        okLabel: '确定'
    })
  }

  return {
    showErrorDialog,
    showSuccessDialog,
    showWarningDialog,
    showInfoDialog
  }
}