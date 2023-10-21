import ReactQuill from 'react-quill';
import 'react-quill/dist/quill.snow.css';
import { useState} from 'react'

export default function PublishNewsLetter() {

  const [body, setBody] = useState('');
    const [subject, setSubject] = useState('')

  const submit = () => {
      const msg = `{"subject": "${subject}", "body": "${body}"}`
      fetch('http://localhost:3000/api/publish', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: msg,
      }).then(res => res.json()).then(data => {
          console.log(data)})
      console.log(msg)
  }

  return (
    <>
        <label>Subject: </label>
        <input type="text" onChange={e=>{setSubject(e.target.value)}}/>
        <ReactQuill theme="snow" value={body} onChange={setBody} />
        <button onClick={submit}>submit</button>
    </>
  )
}
