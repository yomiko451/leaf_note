import {ask, message} from '@tauri-apps/api/dialog'

export default function useDialog() {
  const showErrorDialog = (msg: string) => {
    console.log(msg)  //TODO:dialog
  }

  const showSuccessDialog = async (msg: string) => {
    await message(msg, {
      type: 'info',
      title: '提醒！',
      okLabel: '确定'
  })
  }

  const showAskDialog = async (msg: string) => {
    return await ask(msg, {
        title: '提示',
        type: 'info',
        okLabel: '确定',
        cancelLabel: '取消'
    })
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
    showAskDialog
  }
}