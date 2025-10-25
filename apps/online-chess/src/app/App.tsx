import { Route, Routes } from 'react-router-dom';
import { LandingPage } from './landing_page/LandingPage';
import { SignInModal } from './sign_in/SignInModal';

export function App() {
  return (
    <Routes>
      <Route path="/" element={<LandingPage />} />
      <Route path="/sign_in" element={<SignInModal />} />
      <Route path="/sign_up" element={<SignInModal />} />
    </Routes>
  );
}
