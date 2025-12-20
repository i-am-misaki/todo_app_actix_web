/**
 * 削除確認用のモーダルを表示する。
 * ユーザーが「削除」を確定した場合のみ、実際の削除APIを呼び出す。
 *
 * @param {number} taskId - 削除対象となるタスクのID
 */
function DeleteConfirm(taskId){
    Swal.fire({
      title: "本当に削除していいと？",
      text: "元に戻せんよ！",
      icon: "warning",
      showCancelButton: true,
      confirmButtonColor: "#3085d6",
      cancelButtonColor: "#d33",
      confirmButtonText: "削除",
      cancelButtonText: "やっぱやめる"
    }).then((result) => {
      if (result.isConfirmed) {
        DeleteTaskApi(taskId)
      }
    });
}


/**
 * 指定されたタスクIDをサーバーに送信し、タスクを削除する。
 * 削除完了後は一覧を再読み込みする。
 *
 * @param {number} taskId - 削除対象となるタスクのID
 */
function DeleteTaskApi(taskId){
    fetch('/todo/delete', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: `id=${taskId}`
    }).then(() => {
        console.log('タスクは削除されました。');
        location.reload();
    }).catch(err => {
        console.error(err);
    });
}


/**
 * タスクを編集するため、編集用モーダルを表示する。
 * ユーザーが「編集」を確定した場合のみ、実際の編集 API を呼び出す。
 *
 * @param {number} taskId - 編集対象となるタスクのID
 * @param {string} previousTask - 編集対象となるタスク
 */
function DisplayEditModal(taskId, previousTask){
  (async () => {
    const { value: updatedTask } = await Swal.fire({
      title: "タスクを入力してくれ",
      input: "text",
      // inputLabel: "タスク内容",  // 必要の場合コメントアウト解除する
      inputValue: previousTask,
      confirmButtonText: "編集",
      showCancelButton: true,
      cancelButtonText: "やっぱやめる",
      inputValidator: (value) => {
        if (!value) {
          return "なんか入力せないかんよ";
        }
      }
    });
    if (updatedTask) {
      console.log(updatedTask);
      EditTaskApi(taskId, updatedTask);
    }
  })()
}


/**
 * 指定されたタスクID、タスクをサーバーに送信し、タスクを編集する。
 * 編集完了後は一覧を再読み込みする。
 *
 * Note
 *  - 日本語対応のため encodeURIComponent を使う
 *
 * @param {*} taskId - 編集対象となるタスクのID
 * @param {*} task - 編集対象となるタスク
 */
function EditTaskApi(taskId, task){
  fetch('/todo/edit', {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: `id=${encodeURIComponent(taskId)}&task=${encodeURIComponent(task)}`
    }).then(() => {
        console.log('タスクが編集されました。');
        location.reload();
    }).catch(err => {
        console.error(err);
    });
}
