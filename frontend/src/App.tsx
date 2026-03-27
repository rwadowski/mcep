import { BrowserRouter, NavLink, Route, Routes } from 'react-router-dom';
import DefinitionsPage from './pages/DefinitionsPage';
import DefinitionCreatePage from './pages/DefinitionCreatePage';
import DeploymentsPage from './pages/DeploymentsPage';
import FlowPage from './pages/FlowPage';
import './App.css';

export default function App() {
  return (
    <BrowserRouter>
      <nav className="navbar">
        <span className="brand">MCEP</span>
        <NavLink to="/definitions" className={({ isActive }) => isActive ? 'nav-link active' : 'nav-link'}>
          Definitions
        </NavLink>
        <NavLink to="/deployments" className={({ isActive }) => isActive ? 'nav-link active' : 'nav-link'}>
          Deployments
        </NavLink>
      </nav>
      <main>
        <Routes>
          <Route path="/" element={<DefinitionsPage />} />
          <Route path="/definitions" element={<DefinitionsPage />} />
          <Route path="/definitions/new" element={<DefinitionCreatePage />} />
          <Route path="/deployments" element={<DeploymentsPage />} />
          <Route path="/deployments/new" element={<FlowPage />} />
        </Routes>
      </main>
    </BrowserRouter>
  );
}