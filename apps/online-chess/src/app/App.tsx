import { Route, Routes } from 'react-router-dom';
import { LandingPage } from './landing_page/LandingPage';
import { SignInModal } from './auth_modals/SignInModal';
import { SignUpModal } from './auth_modals/SignUpModal';
import { PlayPage } from './play_page/PlayPage';

export function App() {
  return (
    <Routes>
      <Route path="/" element={<LandingPage />} />
      <Route path="/sign_in" element={<SignInModal />} />
      <Route path="/sign_up" element={<SignUpModal />} />
      <Route path="/play" element={<PlayPage />} />
    </Routes>
  );
}
