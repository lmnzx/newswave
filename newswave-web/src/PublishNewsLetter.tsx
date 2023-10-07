import ReactQuill from 'react-quill';
import 'react-quill/dist/quill.snow.css';
import { useState } from 'react'

export default function PublishNewsLetter() {

  const [value, setValue] = useState('');

  const submit = () => {
      const msg = `{ "body": "${value}" }`;
      fetch('http://localhost:3000/api/publish', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: msg,
      }).then(res => res.json()).then(data => {
          console.log(data)})
  }

  return (
    <>
      <ReactQuill theme="snow" value={value} onChange={setValue} />
      <button onClick={submit}>submit</button>
    </>
  )
}
