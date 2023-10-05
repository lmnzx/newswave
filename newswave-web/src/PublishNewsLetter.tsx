import ReactQuill from 'react-quill';
import 'react-quill/dist/quill.snow.css';
import { useState } from 'react'

export default function PublishNewsLetter() {

  const [value, setValue] = useState('');

  const submit = () => {
     const f = fetch('http://localhost:3000/api/publish', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },body: JSON.stringify(`{ body: ${value} }`),
      }).then(res => res.body)
      console.log(f)
  }

  return (
    <>
      <ReactQuill theme="snow" value={value} onChange={setValue} />
      <button onClick={submit}>submit</button>
    </>
  )
}
