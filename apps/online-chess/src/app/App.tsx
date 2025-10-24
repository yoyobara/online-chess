import { Route, Routes } from 'react-router-dom';
import { LandingPage } from './landing_page/LandingPage';

export function App() {
  return (
    <Routes>
      <Route path="/" element={<LandingPage />} />
    </Routes>
  );
}
