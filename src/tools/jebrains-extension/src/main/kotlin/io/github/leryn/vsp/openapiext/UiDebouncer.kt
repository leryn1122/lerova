package io.github.leryn.vsp.openapiext

import com.intellij.openapi.Disposable
import com.intellij.openapi.application.ModalityState
import com.intellij.openapi.application.invokeLater
import com.intellij.openapi.util.Disposer
import com.intellij.util.Alarm

class UiDebouncer(
  private val parentDisposable: Disposable,
  private val delayMillis: Int = 200
) {
  private val alarm = Alarm(Alarm.ThreadToUse.POOLED_THREAD, parentDisposable)

  fun <T> run(onPooledThread: () -> T, onUiThread: (T) -> Unit) {
    if (Disposer.isDisposed(parentDisposable)) return
    alarm.cancelAllRequests()
    alarm.addRequest({
      val r = onPooledThread()
      invokeLater(ModalityState.any()) {
        if (!Disposer.isDisposed(parentDisposable)) {
          onUiThread(r)
        }
      }
    }, delayMillis)
  }
}
