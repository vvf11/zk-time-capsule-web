import { useState, useEffect } from 'react';
import init, { generate_proof } from '../pkg/zk_time_capsule_web';

function App() {
  const [unlockDate, setUnlockDate] = useState('');
  const [message, setMessage] = useState('');
  const [photo, setPhoto] = useState(null);
  const [proof, setProof] = useState(null);

  useEffect(() => {
    init();
  }, []);

  const handlePhotoChange = (e) => {
    setPhoto(e.target.files[0]);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    try {
      const dateNum = parseInt(unlockDate.replace(/-/g, ''), 10);
      const reader = new FileReader();

      if (photo) {
        reader.readAsArrayBuffer(photo);
        reader.onload = async () => {
          const photoData = new Uint8Array(reader.result);
          const proofBytes = generate_proof(dateNum, message, photoData);
          setProof(proofBytes);

          const blob = new Blob([proofBytes], { type: 'application/octet-stream' });
          const url = URL.createObjectURL(blob);
          const link = document.createElement('a');
          link.href = url;
          link.download = 'proof.bin';
          link.click();
        };
      } else {
        const proofBytes = generate_proof(dateNum, message, new Uint8Array(0));
        setProof(proofBytes);

        const blob = new Blob([proofBytes], { type: 'application/octet-stream' });
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = 'proof.bin';
        link.click();
      }
    } catch (error) {
      alert('Ошибка: ' + error);
    }
  };

  return (
    <div className="container">
      <h1>ZK Time Capsule</h1>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label>Дата открытия:</label>
          <input type="date" value={unlockDate} onChange={(e) => setUnlockDate(e.target.value)} required />
        </div>
        <div className="form-group">
          <label>Сообщение:</label>
          <textarea value={message} onChange={(e) => setMessage(e.target.value)} placeholder="Ваше секретное сообщение" required />
        </div>
        <div className="form-group">
          <label>Прикрепить фото (опционально):</label>
          <input type="file" accept="image/*" onChange={handlePhotoChange} />
        </div>
        <button type="submit">Запечатать капсулу</button>
      </form>
      {proof && <p className="success">Капсула запечатана! Доказательство скачано.</p>}
    </div>
  );
}

export default App;
